use std::fs;
use std::path::{Path, PathBuf};

use forms_input_base_shared::{InputBaseBlueprint, PLACEHOLDER};

fn main() -> std::io::Result<()> {
    let blueprint = InputBaseBlueprint::new();
    let hydrate_invocation = format!(
        "use forms_input_base_sycamore::App;\nuse sycamore::prelude::*;\n\n#[wasm_bindgen::prelude::wasm_bindgen(start)]\npub fn start() {{\n    // SSR placeholder `{}` is asserted to verify hydration parity.\n    sycamore::render(|cx| view! {{ cx, App {{}} }});\n}}\n",
        PLACEHOLDER
    );
    let artifacts = blueprint.bootstrap_artifacts("sycamore", &hydrate_invocation);
    let out_root = workspace_root().join("target/forms-input-base/sycamore");
    if out_root.exists() {
        fs::remove_dir_all(&out_root)?;
    }
    fs::create_dir_all(&out_root)?;
    fs::write(out_root.join("ssr.html"), artifacts.ssr_html)?;
    fs::write(out_root.join("hydrate.rs"), artifacts.hydration_stub)?;
    fs::write(out_root.join("README.md"), artifacts.readme)?;
    println!(
        "[forms-input-base:sycamore] bootstrap assets written to {}",
        out_root.display()
    );
    Ok(())
}

fn workspace_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(3)
        .expect("workspace root")
        .to_path_buf()
}
