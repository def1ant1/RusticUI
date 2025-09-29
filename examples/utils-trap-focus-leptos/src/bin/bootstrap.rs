use std::fs;
use std::path::{Path, PathBuf};

use utils_trap_focus_core::{enterprise_story, TrapFocusStory};

fn main() -> std::io::Result<()> {
    let story = enterprise_story();
    let out_root = workspace_root().join("target/utils-trap-focus/leptos");
    if out_root.exists() {
        fs::remove_dir_all(&out_root)?;
    }
    fs::create_dir_all(&out_root)?;

    fs::write(out_root.join("ssr.html"), story.ssr_document())?;
    fs::write(out_root.join("hydrate.rs"), hydration_stub(&story))?;
    fs::write(out_root.join("README.md"), framework_readme(&story))?;

    println!(
        "[bootstrap:leptos] wrote SSR snapshot and hydration harness to {}",
        out_root.display()
    );
    Ok(())
}

fn hydration_stub(story: &TrapFocusStory) -> String {
    format!(
        "use leptos::prelude::*;\nuse utils_trap_focus_leptos::TrapFocusHarness;\n\nfn main() {{\n    // Analytics `{analytics}` is mirrored to both sentinels so SSR/CSR parity monitors remain stable.\n    mount_to_body(|| view! { <TrapFocusHarness /> });\n}}\n",
        analytics = story.analytics_tag
    )
}

fn framework_readme(story: &TrapFocusStory) -> String {
    format!(
        "# Leptos focus trap bootstrap\n\n\
Generated via `cargo run --bin bootstrap` from `examples/utils-trap-focus-leptos`.\n\
- `ssr.html` preserves the start/end sentinels with automation prefix `{prefix}`.\n\
- `hydrate.rs` mounts `TrapFocusHarness` using Leptos' `mount_to_body`, keeping analytics `{analytics}` intact for observability.\n\n",
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
