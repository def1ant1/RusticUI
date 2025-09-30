use std::fs;
use std::path::{Path, PathBuf};

use forms_input_base_shared::{InputBaseBlueprint, CONTROLLED_ANALYTICS_ID};

fn main() -> std::io::Result<()> {
    let blueprint = InputBaseBlueprint::new();
    let hydrate_invocation = format!(
        "use forms_input_base_yew::App;\nuse yew::Renderer;\n\nfn main() {{\n    // The controlled analytics id `{}` is asserted in automation to ensure hydration kept the namespace intact.\n    Renderer::<App>::new().render();\n}}\n",
        CONTROLLED_ANALYTICS_ID
    );
    let artifacts = blueprint.bootstrap_artifacts("yew", &hydrate_invocation);
    let out_root = workspace_root().join("target/forms-input-base/yew");
    if out_root.exists() {
        fs::remove_dir_all(&out_root)?;
    }
    fs::create_dir_all(&out_root)?;
    fs::write(out_root.join("ssr.html"), artifacts.ssr_html)?;
    fs::write(out_root.join("hydrate.rs"), artifacts.hydration_stub)?;
    fs::write(out_root.join("README.md"), artifacts.readme)?;
    println!(
        "[forms-input-base:yew] bootstrap assets written to {}",
        out_root.display()
    );
    Ok(())
}

fn workspace_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(3)
        .expect("xtask workspace root")
        .to_path_buf()
}
