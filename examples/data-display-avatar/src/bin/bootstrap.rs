use std::fs;
use std::path::{Path, PathBuf};

use data_display_avatar::enterprise_story;
use rustic_ui_system::theme_provider::material_css_baseline_from_theme;

fn main() -> std::io::Result<()> {
    let story = enterprise_story();
    let repo_root = workspace_root();
    let out_root = repo_root.join("target/data-display-avatar");
    if out_root.exists() {
        fs::remove_dir_all(&out_root)?;
    }
    fs::create_dir_all(&out_root)?;

    for (framework, html) in &story.markup {
        let framework_dir = out_root.join(framework);
        fs::create_dir_all(&framework_dir)?;
        fs::write(
            framework_dir.join("ssr.html"),
            ssr_document(&story.theme, html, &story.automation_id),
        )?;
        fs::write(
            framework_dir.join("hydrate.rs"),
            hydration_stub(framework, &story.automation_id),
        )?;
        fs::write(
            framework_dir.join("README.md"),
            framework_readme(framework, &story.automation_id),
        )?;
    }

    println!("Generated avatar bootstrap under {}", out_root.display());
    Ok(())
}

fn workspace_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(2)
        .expect("workspace root")
        .to_path_buf()
}

fn ssr_document(theme: &rustic_ui_styled_engine::Theme, body: &str, automation_id: &str) -> String {
    let baseline = material_css_baseline_from_theme(theme);
    format!(
        "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n    <title>Avatar SSR snapshot</title>\n    <style>{baseline}</style>\n  </head>\n  <body data-rustic-avatar-root=\"rustic-avatar-{automation_id}\">\n    <main id=\"app\">{body}</main>\n  </body>\n</html>\n",
        baseline = baseline,
        automation_id = automation_id,
        body = body
    )
}

fn hydration_stub(framework: &str, automation_id: &str) -> String {
    match framework {
        "yew" => {
            let mut stub = String::new();
            stub.push_str(
                r#"use rustic_ui_styled_engine::ThemeProvider;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let story = data_display_avatar::enterprise_story();
    let markup = story.markup["yew"].clone();
    html! {
        <ThemeProvider theme={story.theme.clone()}>
            <section id="avatar">{ Html::from_html_unchecked(AttrValue::from(markup)) }</section>
        </ThemeProvider>
    }
}

fn main() {
"#,
            );
            stub.push_str(&format!(
                "    tracing::info!(\"hydrate avatar\", automation = \"{}\");\n",
                automation_id
            ));
            stub.push_str(
                r#"    yew::Renderer::<App>::new().render();
}
"#,
            );
            stub
        }
        "leptos" => r#"use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let story = data_display_avatar::enterprise_story();
    let markup = story.markup["leptos"].clone();
    view! { cx,
        <rustic_ui_styled_engine::ThemeProvider theme=story.theme.clone()>
            <section id="avatar" inner_html=markup></section>
        </rustic_ui_styled_engine::ThemeProvider>
    }
}

fn main() {
    leptos::mount_to_body(App);
}
"#
        .to_string(),
        "dioxus" => r#"use dioxus::prelude::*;

fn main() {
    let story = data_display_avatar::enterprise_story();
    let markup = story.markup["dioxus"].clone();
    LaunchBuilder::new(move || VirtualDom::new(|cx| render! { rsx! {
        div { id: "avatar", dangerous_inner_html: markup.clone() }
    }}))
        .launch();
}
"#
        .to_string(),
        "sycamore" => r#"use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let story = data_display_avatar::enterprise_story();
    let markup = story.markup["sycamore"].clone();
    view! { cx,
        section(id="avatar", dangerously_set_inner_html = markup)
    }
}

fn main() {
    sycamore::render(|cx| view! { cx, <App/> });
}
"#
        .to_string(),
        _ => String::new(),
    }
}

fn framework_readme(framework: &str, automation_id: &str) -> String {
    format!(
        "# {framework} avatar bootstrap\n\n\
Run `cargo run --bin bootstrap` inside `examples/data-display-avatar` to regenerate this folder.\n\
The `ssr.html` file contains the avatar chip + tooltip markup for SSR pipelines while `hydrate.rs` shows how to mount the component tree.\n\
Automation anchor: `{automation_id}`\n\
"
    )
}
