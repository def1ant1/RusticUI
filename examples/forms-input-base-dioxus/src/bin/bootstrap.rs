use std::fs;
use std::path::{Path, PathBuf};

use forms_input_base_shared::{InputBaseBlueprint, HYDRATION_NOTE};

fn main() -> std::io::Result<()> {
    let blueprint = InputBaseBlueprint::new();
    let hydrate_invocation = format!(
        "use forms_input_base_dioxus::app;\nuse dioxus_web::Config;\n\nfn main() {{\n    // {note}\n    dioxus_web::launch_cfg(app, Config::default());\n}}\n",
        note = HYDRATION_NOTE
    );
    let artifacts = blueprint.bootstrap_artifacts("dioxus", &hydrate_invocation);
    let out_root = workspace_root().join("target/forms-input-base/dioxus");
    if out_root.exists() {
        fs::remove_dir_all(&out_root)?;
    }
    fs::create_dir_all(&out_root)?;
    fs::write(out_root.join("ssr.html"), artifacts.ssr_html)?;
    fs::write(out_root.join("hydrate.rs"), artifacts.hydration_stub)?;
    fs::write(out_root.join("README.md"), artifacts.readme)?;
    println!(
        "[forms-input-base:dioxus] bootstrap assets written to {}",
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
