use std::fs;
use std::path::{Path, PathBuf};

use utils_trap_focus_core::{enterprise_story, TrapFocusStory};
use utils_trap_focus_dioxus::trap_focus_markup;

fn main() -> std::io::Result<()> {
    let story = enterprise_story();
    let out_root = workspace_root().join("target/utils-trap-focus/dioxus");
    if out_root.exists() {
        fs::remove_dir_all(&out_root)?;
    }
    fs::create_dir_all(&out_root)?;

    fs::write(out_root.join("ssr.html"), story.ssr_document())?;
    fs::write(out_root.join("hydrate.rs"), hydration_stub(&story))?;
    fs::write(out_root.join("README.md"), framework_readme(&story))?;

    println!(
        "[bootstrap:dioxus] wrote SSR snapshot and hydration harness to {}",
        out_root.display()
    );
    Ok(())
}

fn hydration_stub(story: &TrapFocusStory) -> String {
    format!(
        "use dioxus::prelude::*;\nuse utils_trap_focus_dioxus::TrapFocusHarness;\n\nfn main() {{\n    // Launch the shared harness so analytics `{analytics}` stay stable across SSR + hydration.\n    dioxus_web::launch(TrapFocusHarness);\n}}\n",
        analytics = story.analytics_tag
    )
}

fn framework_readme(story: &TrapFocusStory) -> String {
    format!(
        "# Dioxus focus trap bootstrap\n\n\
Generated via `cargo run --bin bootstrap` from `examples/utils-trap-focus-dioxus`.\n\
- `ssr.html` mirrors `trap_focus_markup()` so QA agents diff the same markup Dioxus hydrates.\n\
- `hydrate.rs` launches `TrapFocusHarness`, preserving analytics `{analytics}` and automation prefix `{prefix}`.\n\n",
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
