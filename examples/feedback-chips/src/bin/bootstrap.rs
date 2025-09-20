use std::fs;
use std::path::{Path, PathBuf};

use feedback_chips::enterprise_story;
use mui_system::theme_provider::material_css_baseline_from_theme;

fn main() -> std::io::Result<()> {
    let story = enterprise_story();
    let repo_root = workspace_root();
    let out_root = repo_root.join("target/feedback-chips");
    if out_root.exists() {
        fs::remove_dir_all(&out_root)?;
    }
    fs::create_dir_all(&out_root)?;

    for (framework, html) in &story.dismissible {
        let framework_dir = out_root.join(framework);
        fs::create_dir_all(&framework_dir)?;
        let dismissible = ssr_document(&story.theme, html, &story.automation_id);
        fs::write(framework_dir.join("dismissible.html"), dismissible)?;
        if let Some(read_only_html) = story.read_only.get(framework) {
            let read_only = ssr_document(
                &story.theme,
                read_only_html,
                &format!("{}-static", story.automation_id),
            );
            fs::write(framework_dir.join("read-only.html"), read_only)?;
        }
        fs::write(
            framework_dir.join("hydrate.rs"),
            hydration_stub(framework, &story.automation_id),
        )?;
        fs::write(
            framework_dir.join("README.md"),
            framework_readme(framework, &story.automation_id),
        )?;
    }

    println!("Generated chip bootstrap under {}", out_root.display());
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

fn ssr_document(theme: &mui_styled_engine::Theme, body: &str, automation_id: &str) -> String {
    let baseline = material_css_baseline_from_theme(theme);
    format!(
        "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\" />\n    <title>Chip SSR snapshot</title>\n    <style>{baseline}</style>\n  </head>\n  <body data-automation-root=\"{automation_id}\">\n    <div id=\"app\">{body}</div>\n  </body>\n</html>\n",
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
                r#"use mui_material::chip::yew;
use mui_styled_engine::ThemeProvider;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let story = feedback_chips::enterprise_story();
    let dismissible = story.dismissible["yew"].clone();
    let read_only = story.read_only["yew"].clone();
    html! {
        <ThemeProvider theme={story.theme.clone()}>
            <div id="dismissible">{ Html::from_html_unchecked(AttrValue::from(dismissible)) }</div>
            <div id="read-only">{ Html::from_html_unchecked(AttrValue::from(read_only)) }</div>
        </ThemeProvider>
    }
}

fn main() {
"#,
            );
            stub.push_str(&format!(
                "    tracing::info!(\"hydrate chips\", automation = \"{}\");\n",
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
    let story = feedback_chips::enterprise_story();
    let dismissible = story.dismissible["leptos"].clone();
    let read_only = story.read_only["leptos"].clone();
    view! { cx,
        <mui_styled_engine::ThemeProvider theme=story.theme.clone()>
            <section id="dismissible" inner_html=dismissible></section>
            <section id="read-only" inner_html=read_only></section>
        </mui_styled_engine::ThemeProvider>
    }
}

fn main() {
    leptos::mount_to_body(App);
}
"#
        .to_string(),
        "dioxus" => r#"use dioxus::prelude::*;

fn main() {
    let story = feedback_chips::enterprise_story();
    let dismissible = story.dismissible["dioxus"].clone();
    let read_only = story.read_only["dioxus"].clone();
    LaunchBuilder::new(move || VirtualDom::new(|cx| render! { rsx! {
        div { id: "app",
            div { id: "dismissible", dangerous_inner_html: dismissible.clone() }
            div { id: "read-only", dangerous_inner_html: read_only.clone() }
        }
    }}))
        .launch();
}
"#
        .to_string(),
        "sycamore" => r#"use sycamore::prelude::*;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let story = feedback_chips::enterprise_story();
    let dismissible = story.dismissible["sycamore"].clone();
    let read_only = story.read_only["sycamore"].clone();
    view! { cx,
        div(id="app")
            div(id="dismissible", dangerously_set_inner_html = dismissible)
            div(id="read-only", dangerously_set_inner_html = read_only)
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
        "# {framework} chip bootstrap\n\n\
Generated via `cargo run --bin bootstrap` from `examples/feedback-chips`.\n\
`dismissible.html` and `read-only.html` contain SSR markup for the two chip variants.\n\
Wrap the hydration root with the returned theme so hover/focus affordances match the server snapshot.\n\
Automation ids:\n\
- Dismissible: `{automation_id}`\n- Read-only: `{automation_id}-static`\n\n"
    )
}
