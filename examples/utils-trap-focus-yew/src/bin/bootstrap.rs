use std::fs;
use std::path::{Path, PathBuf};

use utils_trap_focus_core::{enterprise_story, TrapFocusStory};

fn main() -> std::io::Result<()> {
    let story = enterprise_story();
    let out_root = workspace_root().join("target/utils-trap-focus/yew");
    if out_root.exists() {
        fs::remove_dir_all(&out_root)?;
    }
    fs::create_dir_all(&out_root)?;

    fs::write(out_root.join("ssr.html"), story.ssr_document())?;
    fs::write(out_root.join("hydrate.rs"), hydration_stub(&story))?;
    fs::write(out_root.join("README.md"), framework_readme(&story))?;

    println!(
        "[bootstrap:yew] wrote SSR snapshot and hydration harness to {}",
        out_root.display()
    );
    Ok(())
}

fn hydration_stub(story: &TrapFocusStory) -> String {
    format!(
        "use utils_trap_focus_yew::TrapFocusHarness;\nuse yew::Renderer;\n\nfn main() {{\n    // The analytics tag `{analytics}` is mirrored onto the sentinels so\n    // dashboards can confirm the trap stayed active after hydration.\n    Renderer::<TrapFocusHarness>::new().render();\n}}\n",
        analytics = story.analytics_tag
    )
}

fn framework_readme(story: &TrapFocusStory) -> String {
    format!(
        "# Yew focus trap bootstrap\n\n\
This directory is generated via `cargo run --bin bootstrap` from `examples/utils-trap-focus-yew`.\n\
- `ssr.html` captures the shared SSR markup with deterministic `data-rustic-focus-trap` hooks.\n\
- `hydrate.rs` mounts `TrapFocusHarness` which reuses the same `FocusTrapState` analytics tag (`{analytics}`) for parity checks.\n\
- Automation prefix: `{prefix}` – expect sentinels named `{prefix}::focus-trap-start` / `{prefix}::focus-trap-end`.\n\n",
        analytics = story.analytics_tag,
        prefix = story.automation_prefix,
    )
}

fn workspace_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(3)
        .expect("workspace root")
        .to_path_buf()
}
