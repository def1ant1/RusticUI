use std::fs;
use std::path::{Path, PathBuf};

use forms_input_base_shared::{InputBaseBlueprint, UNCONTROLLED_ANALYTICS_ID};

fn main() -> std::io::Result<()> {
    let blueprint = InputBaseBlueprint::new();
    let hydrate_invocation = format!(
        "use forms_input_base_leptos::App;\nuse leptos::*;\n\nfn main() {{\n    // Hydration asserts the uncontrolled analytics id `{}` to confirm the state machine survived SSR.\n    mount_to_body(|| view! {{ <App/> }});\n}}\n",
        UNCONTROLLED_ANALYTICS_ID
    );
    let artifacts = blueprint.bootstrap_artifacts("leptos", &hydrate_invocation);
    let out_root = workspace_root().join("target/forms-input-base/leptos");
    if out_root.exists() {
        fs::remove_dir_all(&out_root)?;
    }
    fs::create_dir_all(&out_root)?;
    fs::write(out_root.join("ssr.html"), artifacts.ssr_html)?;
    fs::write(out_root.join("hydrate.rs"), artifacts.hydration_stub)?;
    fs::write(out_root.join("README.md"), artifacts.readme)?;
    println!(
        "[forms-input-base:leptos] bootstrap assets written to {}",
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
