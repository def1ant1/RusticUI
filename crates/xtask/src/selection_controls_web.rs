use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{anyhow, bail, Context, Result};
use headless_chrome::{Browser, LaunchOptionsBuilder};
use serde::Serialize;

/// Declarative configuration describing each framework participating in the
/// selection controls showcase.
///
/// The Node Playwright harness used a JavaScript object to encode the same
/// matrix.  Keeping the shape identical ensures CI and local automation touch
/// the exact same demos as the pre-migration pipeline.  The struct-based
/// approach lets us derive strongly typed tests that guard against accidental
/// drift when new frameworks are onboarded.
#[derive(Debug, Clone)]
struct FrameworkSpec {
    name: &'static str,
    port: u16,
    base_url: &'static str,
    path: &'static str,
    automation_keys: &'static [&'static str],
}

impl FrameworkSpec {
    fn target_url(&self) -> String {
        format!("{}:{}{}", self.base_url, self.port, self.path)
    }
}

const FRAMEWORKS: &[FrameworkSpec] = &[
    FrameworkSpec {
        name: "dioxus",
        port: 4701,
        base_url: "http://127.0.0.1",
        path: "/",
        automation_keys: &["checkbox", "switch", "radio", "telemetry-log"],
    },
    FrameworkSpec {
        name: "sycamore",
        port: 4702,
        base_url: "http://127.0.0.1",
        path: "/",
        automation_keys: &["checkbox", "switch", "radio", "telemetry-log"],
    },
    FrameworkSpec {
        name: "yew",
        port: 4703,
        base_url: "http://127.0.0.1",
        path: "/",
        automation_keys: &["checkbox", "switch", "radio", "telemetry-log"],
    },
    FrameworkSpec {
        name: "react",
        port: 4704,
        base_url: "http://127.0.0.1",
        path: "/",
        automation_keys: &["checkbox", "switch", "telemetry-log"],
    },
];

/// Persisted audit events emitted by the harness.  Tests assert on these JSON
/// entries to prove we exercise the full matrix even when the actual browser is
/// stubbed out.
#[derive(Debug, Serialize)]
struct AuditEvent<'a> {
    framework: &'a str,
    selectors: &'a [SelectorSnapshot],
    status: &'a str,
}

#[derive(Debug, Serialize)]
struct SelectorSnapshot {
    key: &'static str,
    automation_id: String,
}

/// Optional JSONL recorder for tests and CI instrumentation.  When the
/// `RUSTICUI_SELECTION_CONTROLS_AUDIT_LOG` environment variable is set the
/// harness appends one JSON object per framework.
struct AuditRecorder {
    file: Option<File>,
}

impl AuditRecorder {
    fn new() -> Result<Self> {
        match env::var("RUSTICUI_SELECTION_CONTROLS_AUDIT_LOG") {
            Ok(path) => {
                let file = File::create(path).context("failed to create automation audit log")?;
                Ok(Self { file: Some(file) })
            }
            Err(_) => Ok(Self { file: None }),
        }
    }

    fn record(&mut self, event: AuditEvent<'_>) -> Result<()> {
        if let Some(file) = self.file.as_mut() {
            let payload = serde_json::to_string(&event)
                .context("failed to serialize automation audit event")?;
            writeln!(file, "{}", payload).context("failed to write automation audit event")?;
        }
        Ok(())
    }
}

/// RAII guard that keeps the dev server process alive while Playwright-style
/// automation runs and tears it down even if the harness panics.
struct ServerGuard {
    child: Option<Child>,
    stdout_handle: Option<thread::JoinHandle<()>>,
    stderr_handle: Option<thread::JoinHandle<()>>,
}

impl ServerGuard {
    fn spawn(spec: &FrameworkSpec, workspace: &Path, smoke_helper: &Path) -> Result<Self> {
        let mut command = Command::new(smoke_helper);
        command
            .current_dir(workspace)
            .arg(spec.name)
            .arg("--mode")
            .arg("serve")
            .arg("--port")
            .arg(spec.port.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        let mut child = command
            .spawn()
            .with_context(|| format!("failed to launch {} selection controls server", spec.name))?;

        let stdout_handle = child.stdout.take().map(|stdout| {
            spawn_pipe_reader(
                stdout,
                format!("[selection-controls][{}][server][stdout]", spec.name),
            )
        });
        let stderr_handle = child.stderr.take().map(|stderr| {
            spawn_pipe_reader(
                stderr,
                format!("[selection-controls][{}][server][stderr]", spec.name),
            )
        });

        Ok(Self {
            child: Some(child),
            stdout_handle,
            stderr_handle,
        })
    }

    fn shutdown(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            match child.try_wait() {
                Ok(Some(_)) => {
                    let _ = child.wait();
                }
                Ok(None) => {
                    child
                        .kill()
                        .with_context(|| "failed to terminate selection controls dev server")?;
                    let _ = child.wait();
                }
                Err(error) => {
                    return Err(anyhow!(
                        "failed to query selection controls dev server state: {error}"
                    ));
                }
            }
        }
        if let Some(handle) = self.stdout_handle.take() {
            let _ = handle.join();
        }
        if let Some(handle) = self.stderr_handle.take() {
            let _ = handle.join();
        }
        Ok(())
    }
}

impl Drop for ServerGuard {
    fn drop(&mut self) {
        if let Err(error) = self.shutdown() {
            eprintln!("[selection-controls][warn] error while shutting down dev server: {error:?}");
        }
    }
}

fn spawn_pipe_reader<R>(pipe: R, prefix: String) -> thread::JoinHandle<()>
where
    R: 'static + Send + std::io::Read,
{
    thread::spawn(move || {
        let mut reader = BufReader::new(pipe);
        let mut line = String::new();
        while let Ok(bytes) = reader.read_line(&mut line) {
            if bytes == 0 {
                break;
            }
            print!("{} {}", prefix, line);
            line.clear();
        }
    })
}

fn wait_for_server(url: &str, timeout: Duration) -> Result<()> {
    let deadline = Instant::now() + timeout;
    let agent = ureq::AgentBuilder::new()
        .timeout_connect(Duration::from_secs(2))
        .timeout_read(Duration::from_secs(2))
        .build();

    loop {
        match agent.get(url).call() {
            Ok(response) => {
                if response.status() < 500 {
                    return Ok(());
                }
            }
            Err(ureq::Error::Status(code, _)) if code < 500 => {
                return Ok(());
            }
            Err(_) => {}
        }

        if Instant::now() >= deadline {
            bail!("server did not respond at {} within {:?}", url, timeout);
        }

        thread::sleep(Duration::from_millis(250));
    }
}

fn load_automation_lookup(
    workspace: &Path,
    smoke_helper: &Path,
) -> Result<HashMap<&'static str, String>> {
    let output = Command::new(smoke_helper)
        .current_dir(workspace)
        .arg("--format")
        .arg("json")
        .arg("--list-automation")
        .output()
        .context("failed to list selection controls automation IDs")?;

    if !output.status.success() {
        bail!(
            "listing automation identifiers failed with status {:?}",
            output.status
        );
    }

    let ids: Vec<String> = serde_json::from_slice(&output.stdout)
        .context("failed to parse automation identifiers from smoke helper")?;

    let mut mapping = HashMap::new();
    for id in ids {
        if id.ends_with(".checkbox") {
            mapping.insert("checkbox", id);
        } else if id.ends_with(".switch") {
            mapping.insert("switch", id);
        } else if id.ends_with(".radio") {
            mapping.insert("radio", id);
        } else if id.ends_with(".telemetry-log") {
            mapping.insert("telemetry-log", id);
        }
    }

    for key in ["checkbox", "switch", "radio", "telemetry-log"] {
        if !mapping.contains_key(key) {
            bail!(
                "automation identifier for '{}' missing from smoke helper output",
                key
            );
        }
    }

    Ok(mapping)
}

fn run_browser_probe(
    spec: &FrameworkSpec,
    url: &str,
    selectors: &[SelectorSnapshot],
) -> Result<()> {
    let mut launch_builder = LaunchOptionsBuilder::default();
    let chrome_args: Vec<&OsStr> = vec![
        OsStr::new("--headless=new"),
        OsStr::new("--disable-gpu"),
        OsStr::new("--disable-dev-shm-usage"),
        OsStr::new("--disable-background-networking"),
    ];
    launch_builder.args(chrome_args);

    if let Ok(chrome_path) = env::var("RUSTICUI_SELECTION_CONTROLS_CHROME") {
        launch_builder.path(Some(chrome_path.into()));
    }

    let launch_options = launch_builder
        .build()
        .map_err(|error| anyhow!("failed to construct Chrome launch options: {error}"))?;

    let browser = Browser::new(launch_options).context("failed to launch headless Chrome")?;
    let tab = browser
        .new_tab()
        .context("failed to open automation browser tab")?;

    tab.navigate_to(url)
        .with_context(|| format!("failed to navigate {} automation tab to {}", spec.name, url))?;
    tab.wait_until_navigated()
        .with_context(|| format!("{} automation tab never finished loading", spec.name))?;

    for snapshot in selectors {
        let selector = format!("[data-automation-id=\"{}\"]", snapshot.automation_id);
        tab.wait_for_element(&selector).with_context(|| {
            format!(
                "framework '{}' is missing automation selector '{}'",
                spec.name, snapshot.automation_id
            )
        })?;
    }

    Ok(())
}

fn resolve_frameworks(filter: Option<&str>) -> Result<Vec<&'static FrameworkSpec>> {
    match filter {
        None | Some("all") => Ok(FRAMEWORKS.iter().collect()),
        Some(name) => {
            let normalized = name.trim().to_lowercase();
            let framework = FRAMEWORKS
                .iter()
                .find(|framework| framework.name == normalized)
                .ok_or_else(|| {
                    anyhow!(
                        "unknown framework '{}'. supported: {}",
                        name,
                        FRAMEWORKS
                            .iter()
                            .map(|framework| framework.name)
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                })?;
            Ok(vec![framework])
        }
    }
}

/// Rust-first replacement for the historical Node Playwright harness.
///
/// The wrapper bootstraps the same automation matrix but relies exclusively on
/// Rust crates (`headless_chrome` + the existing smoke helper) so that CI
/// environments do not need a Node runtime.  The harness exposes two testing
/// hooks via environment variables:
/// - `RUSTICUI_SELECTION_CONTROLS_SKIP_SERVER` skips provisioning dev servers.
/// - `RUSTICUI_SELECTION_CONTROLS_SKIP_BROWSER` short-circuits headless Chrome
///   execution while still emitting audit events.
///
/// Both switches default to `false` and are only consumed by the integration
/// tests added alongside this migration.
pub struct SelectionControlsHarness {
    workspace: PathBuf,
    smoke_helper: PathBuf,
}

impl SelectionControlsHarness {
    pub fn new(workspace: PathBuf) -> Self {
        let smoke_helper = workspace.join("examples/scripts/selection-controls-smoke.sh");
        Self {
            workspace,
            smoke_helper,
        }
    }

    /// Execute the browser automation matrix.
    ///
    /// CI wires this through `cargo xtask selection-controls` so the same Rust
    /// binary provisions dev servers, waits for the HTTP endpoints, and
    /// validates automation selectors without shelling out to Node.  The
    /// optional `filter` parameter mirrors the historical `--framework` flag so
    /// individual teams can focus on a single renderer during local triage.
    pub fn run(&self, filter: Option<&str>) -> Result<()> {
        let frameworks = resolve_frameworks(filter)?;
        let mut audit = AuditRecorder::new()?;
        let automation_lookup = load_automation_lookup(&self.workspace, &self.smoke_helper)?;
        let skip_server = env::var("RUSTICUI_SELECTION_CONTROLS_SKIP_SERVER").is_ok();
        let skip_browser = env::var("RUSTICUI_SELECTION_CONTROLS_SKIP_BROWSER").is_ok();

        for framework in frameworks {
            let selectors: Vec<SelectorSnapshot> = framework
                .automation_keys
                .iter()
                .map(|key| -> Result<SelectorSnapshot> {
                    Ok(SelectorSnapshot {
                        key,
                        automation_id: automation_lookup.get(key).cloned().ok_or_else(|| {
                            anyhow!(
                                "automation id for '{}' missing while verifying {}",
                                key,
                                framework.name
                            )
                        })?,
                    })
                })
                .collect::<Result<_, _>>()?;

            if skip_server {
                audit.record(AuditEvent {
                    framework: framework.name,
                    selectors: &selectors,
                    status: "skip-server",
                })?;
                continue;
            }

            let mut server = ServerGuard::spawn(framework, &self.workspace, &self.smoke_helper)?;
            let target_url = framework.target_url();
            wait_for_server(&target_url, Duration::from_secs(120))?;

            if skip_browser {
                audit.record(AuditEvent {
                    framework: framework.name,
                    selectors: &selectors,
                    status: "skip-browser",
                })?;
            } else {
                run_browser_probe(framework, &target_url, &selectors)?;
                audit.record(AuditEvent {
                    framework: framework.name,
                    selectors: &selectors,
                    status: "verified",
                })?;
            }

            server.shutdown()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn matrix_matches_historical_playwright_selection() {
        let expected: BTreeSet<&'static str> = ["dioxus", "sycamore", "yew", "react"].into();
        let actual: BTreeSet<&'static str> = FRAMEWORKS.iter().map(|spec| spec.name).collect();
        assert_eq!(
            actual, expected,
            "framework coverage should remain unchanged"
        );
    }

    #[test]
    fn automation_keys_align_with_contract() {
        let expected: BTreeSet<&'static str> =
            ["checkbox", "switch", "radio", "telemetry-log"].into();
        let union: BTreeSet<&'static str> = FRAMEWORKS
            .iter()
            .flat_map(|spec| spec.automation_keys.iter().copied())
            .collect();
        assert_eq!(union, expected, "automation key coverage regressed");
    }
}
