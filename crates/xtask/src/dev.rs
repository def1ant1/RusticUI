use crate::workspace_root;
use anyhow::{anyhow, bail, Context, Result};
use clap::Args;
use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

/// Spin up the docs site and example gallery with a single hot-reload harness.
#[derive(Args, Debug)]
pub struct DevArgs {
    /// Preview the orchestrated commands without spawning long-running processes.
    #[arg(long)]
    pub dry_run: bool,
    /// Hostname bound by the Next.js docs development server.
    #[arg(long, default_value = "127.0.0.1")]
    pub docs_host: String,
    /// Port consumed by the Next.js docs development server.
    #[arg(long, default_value_t = 3100)]
    pub docs_port: u16,
    /// Hostname bound by the Leptos-powered example gallery server.
    #[arg(long, default_value = "127.0.0.1")]
    pub gallery_host: String,
    /// Port consumed by the Leptos-powered example gallery server.
    #[arg(long, default_value_t = 3000)]
    pub gallery_port: u16,
    /// Skip launching the Next.js docs server.
    #[arg(long)]
    pub skip_docs: bool,
    /// Skip launching the Leptos example gallery server.
    #[arg(long)]
    pub skip_gallery: bool,
}

pub fn dev(args: DevArgs) -> Result<()> {
    if args.skip_docs && args.skip_gallery {
        bail!("cannot skip both docs and gallery processes");
    }

    let workspace = workspace_root();
    let plans = plan_commands(&workspace, &args)?;

    if args.dry_run {
        println!("[xtask][dev] dry-run mode");
        for plan in &plans {
            println!("  - {} => {}", plan.label, plan.render_display());
        }
        println!("[xtask][dev] commands ready — run without --dry-run to launch services");
        return Ok(());
    }

    let log = prepare_log(&workspace, &args)?;
    println!("[xtask][dev] log file: {}", log.relative_path());

    let mut running = Vec::new();
    for plan in plans {
        match plan.spawn(log.writer()) {
            Ok(handle) => running.push(handle),
            Err(err) => {
                // Attempt to terminate already spawned processes before bubbling the error.
                for mut process in running {
                    process.shutdown();
                }
                return Err(err);
            }
        }
    }

    // Block until one of the processes exits. This mirrors typical dev server behaviour
    // where Ctrl+C is used to terminate the harness.
    for process in &mut running {
        let status = process.wait()?;
        if !status.success() {
            return Err(anyhow!(
                "{} command exited with status {:?}",
                process.label,
                status
            ));
        }
    }

    // Join background reader threads to avoid premature log truncation.
    for mut process in running {
        process.join_threads();
    }

    Ok(())
}

fn plan_commands(workspace: &Path, args: &DevArgs) -> Result<Vec<DevCommandPlan>> {
    let mut plans = Vec::new();
    if !args.skip_docs {
        plans.push(DevCommandPlan::docs(workspace, args)?);
    }
    if !args.skip_gallery {
        plans.push(DevCommandPlan::gallery(workspace, args)?);
    }
    if plans.is_empty() {
        bail!("no processes scheduled");
    }
    Ok(plans)
}

struct DevCommandPlan {
    label: &'static str,
    program: String,
    args: Vec<String>,
    cwd: PathBuf,
    env: Vec<(String, String)>,
}

impl DevCommandPlan {
    fn docs(workspace: &Path, args: &DevArgs) -> Result<Self> {
        let mut plan = Self {
            label: "docs",
            program: "pnpm".into(),
            args: vec![
                "--dir".into(),
                "docs".into(),
                "run".into(),
                "dev".into(),
                "--".into(),
                "--hostname".into(),
                args.docs_host.clone(),
                "--port".into(),
                args.docs_port.to_string(),
            ],
            cwd: workspace.to_path_buf(),
            env: Vec::new(),
        };
        plan.env.push((
            "PLAYWRIGHT_TEST_BASE_URL".into(),
            format!("http://{}:{}", args.docs_host, args.docs_port),
        ));
        Ok(plan)
    }

    fn gallery(workspace: &Path, args: &DevArgs) -> Result<Self> {
        let target_dir = workspace.join("target/dev");
        let listen_addr = format!("{}:{}", args.gallery_host, args.gallery_port);
        Ok(Self {
            label: "gallery",
            program: "cargo".into(),
            args: vec![
                "run".into(),
                "-p".into(),
                "rustic-docs".into(),
                "--bin".into(),
                "rustic-docs-server".into(),
                "--features".into(),
                "ssr".into(),
            ],
            cwd: workspace.to_path_buf(),
            env: vec![
                ("CARGO_TARGET_DIR".into(), target_dir.display().to_string()),
                ("RUST_LOG".into(), "info,rustic_docs=debug".into()),
                ("LEPTOS_SITE_ADDR".into(), listen_addr),
            ],
        })
    }

    fn spawn(self, log: Arc<Mutex<BufWriter<fs::File>>>) -> Result<RunningProcess> {
        let mut cmd = Command::new(&self.program);
        cmd.args(&self.args)
            .current_dir(&self.cwd)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        for (key, value) in &self.env {
            cmd.env(key, value);
        }

        // Record the spawn event in the shared log before launching the process.
        {
            let mut guard = log
                .lock()
                .expect("dev log mutex poisoned while recording spawn");
            writeln!(
                guard,
                "command label={label} program={program} args={args}",
                label = self.label,
                program = self.program,
                args = self.render_display()
            )?;
            guard.flush()?;
        }

        let mut child = cmd
            .spawn()
            .with_context(|| format!("failed to spawn {}", self.render_display()))?;

        let stdout = child
            .stdout
            .take()
            .map(|stream| spawn_reader_thread(self.label, log.clone(), stream, StreamKind::Stdout));
        let stderr = child
            .stderr
            .take()
            .map(|stream| spawn_reader_thread(self.label, log.clone(), stream, StreamKind::Stderr));

        println!(
            "[xtask][dev] launched {} => {}",
            self.label,
            self.render_display()
        );

        Ok(RunningProcess {
            label: self.label,
            child,
            stdout,
            stderr,
        })
    }

    fn render_display(&self) -> String {
        let mut repr = quote_segment(&self.program);
        for arg in &self.args {
            repr.push(' ');
            repr.push_str(&quote_segment(arg));
        }
        repr
    }
}

fn quote_segment(segment: &str) -> String {
    if segment.is_empty()
        || segment.contains(|c: char| c.is_whitespace() || matches!(c, '"' | '\''))
    {
        format!("\"{}\"", segment.replace('"', "\\\""))
    } else {
        segment.to_string()
    }
}

struct RunningProcess {
    label: &'static str,
    child: Child,
    stdout: Option<thread::JoinHandle<()>>,
    stderr: Option<thread::JoinHandle<()>>,
}

impl RunningProcess {
    fn wait(&mut self) -> Result<std::process::ExitStatus> {
        self.child.wait().map_err(|err| err.into())
    }

    fn join_threads(&mut self) {
        if let Some(handle) = self.stdout.take() {
            let _ = handle.join();
        }
        if let Some(handle) = self.stderr.take() {
            let _ = handle.join();
        }
    }

    fn shutdown(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        self.join_threads();
    }
}

impl Drop for RunningProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[derive(Debug, Copy, Clone)]
enum StreamKind {
    Stdout,
    Stderr,
}

fn spawn_reader_thread(
    label: &'static str,
    log: Arc<Mutex<BufWriter<fs::File>>>,
    stream: impl std::io::Read + Send + 'static,
    kind: StreamKind,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let reader = BufReader::new(stream);
        for line in reader.lines().flatten() {
            println!("[xtask][dev][{label}] {line}");
            if let Ok(mut guard) = log.lock() {
                let _ = writeln!(guard, "stream label={label} kind={kind:?} line={line}",);
                let _ = guard.flush();
            }
        }
    })
}

struct DevLog {
    path: PathBuf,
    writer: Arc<Mutex<BufWriter<fs::File>>>,
}

impl DevLog {
    fn relative_path(&self) -> String {
        let workspace = workspace_root();
        self.path
            .strip_prefix(&workspace)
            .map(|path| path.display().to_string())
            .unwrap_or_else(|_| self.path.display().to_string())
    }

    fn writer(&self) -> Arc<Mutex<BufWriter<fs::File>>> {
        self.writer.clone()
    }
}

fn prepare_log(workspace: &Path, args: &DevArgs) -> Result<DevLog> {
    let logs_dir = workspace.join("target/logs");
    fs::create_dir_all(&logs_dir)
        .with_context(|| format!("failed to create log directory {}", logs_dir.display()))?;
    let log_path = logs_dir.join("dev.log");
    let file = fs::File::create(&log_path)
        .with_context(|| format!("failed to create log file {}", log_path.display()))?;
    let mut writer = BufWriter::new(file);
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    writeln!(
        writer,
        "# cargo xtask dev session\nstarted_at_unix={ts}\ndocs_host={}:{}\ngallery_host={}:{}\n",
        args.docs_host, args.docs_port, args.gallery_host, args.gallery_port
    )?;
    writer.flush()?;
    Ok(DevLog {
        path: log_path,
        writer: Arc::new(Mutex::new(writer)),
    })
}
