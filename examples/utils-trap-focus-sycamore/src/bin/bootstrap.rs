use std::fs;
use std::path::{Path, PathBuf};

use utils_trap_focus_core::{enterprise_story, TrapFocusStory};

fn main() -> std::io::Result<()> {
    let story = enterprise_story();
    let out_root = workspace_root().join("target/utils-trap-focus/sycamore");
    if out_root.exists() {
        fs::remove_dir_all(&out_root)?;
    }
    fs::create_dir_all(&out_root)?;

    fs::write(out_root.join("ssr.html"), story.ssr_document())?;
    fs::write(out_root.join("hydrate.rs"), hydration_stub(&story))?;
    fs::write(out_root.join("README.md"), framework_readme(&story))?;

    println!(
        "[bootstrap:sycamore] wrote SSR snapshot and hydration harness to {}",
        out_root.display()
    );
    Ok(())
}

fn hydration_stub(story: &TrapFocusStory) -> String {
    format!(
        "use sycamore::prelude::*;\nuse utils_trap_focus_sycamore::TrapFocusHarness;\n\nfn main() {{\n    // Hydrate the shared snapshot and retain analytics `{analytics}` for monitoring.\n    sycamore::render(|cx| view! { cx, <TrapFocusHarness /> });\n}}\n",
        analytics = story.analytics_tag
    )
}

fn framework_readme(story: &TrapFocusStory) -> String {
    format!(
        "# Sycamore focus trap bootstrap\n\n\
Generated via `cargo run --bin bootstrap` from `examples/utils-trap-focus-sycamore`.\n\
- `ssr.html` mirrors the shared story with automation prefix `{prefix}`.\n\
- `hydrate.rs` renders `TrapFocusHarness` directly so analytics `{analytics}` stay aligned with other frameworks.\n\n",
        prefix = story.automation_prefix,
        analytics = story.analytics_tag,
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
