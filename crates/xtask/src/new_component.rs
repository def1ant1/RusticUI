use crate::workspace_root;
use anyhow::{bail, Context, Result};
use clap::Args;
use heck::{ToKebabCase, ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use std::fs;
use std::path::{Path, PathBuf};

/// Arguments for the `cargo xtask new-component` generator.
#[derive(Args, Debug)]
pub struct NewComponentArgs {
    /// Component name to scaffold. Accepts PascalCase, snake_case, or kebab-case.
    pub name: String,
    /// Only emit Material scaffolding (Rust + TypeScript).
    #[arg(long, conflicts_with = "headless_only")]
    pub material_only: bool,
    /// Only emit headless scaffolding.
    #[arg(long)]
    pub headless_only: bool,
    /// Preview the generated files without touching disk.
    #[arg(long)]
    pub dry_run: bool,
    /// Overwrite existing files instead of failing.
    #[arg(long)]
    pub overwrite: bool,
}

pub fn new_component(args: NewComponentArgs) -> Result<()> {
    if args.name.trim().is_empty() {
        bail!("component name cannot be empty");
    }

    let component_pascal = args.name.to_upper_camel_case();
    if component_pascal.is_empty() {
        bail!("failed to normalise component name `{}`", args.name);
    }
    let component_snake = component_pascal.to_snake_case();
    let component_kebab = component_pascal.to_kebab_case();
    let component_shouty = component_pascal.to_shouty_snake_case();
    let automation_id = format!("automation.{}", component_kebab);

    let workspace = workspace_root();
    let template_root = workspace.join("tools/templates/new-component");
    if !template_root.exists() {
        bail!(
            "template directory `{}` missing; ensure repository templates were committed",
            template_root.display()
        );
    }

    let ctx = TemplateContext {
        component_pascal,
        component_snake,
        component_kebab,
        component_shouty,
        automation_id,
    };

    let audience = if args.material_only {
        TemplateAudience::Material
    } else if args.headless_only {
        TemplateAudience::Headless
    } else {
        TemplateAudience::All
    };

    let specs = build_template_specs(&workspace, &template_root, &ctx, audience)?;
    if args.dry_run {
        println!(
            "[xtask][new-component] dry-run mode for `{}`",
            ctx.component_pascal
        );
        for spec in &specs {
            println!(
                "  - would write {} ({})",
                spec.output.display(),
                spec.description
            );
        }
        println!(
            "[xtask][new-component] templates ready — run without --dry-run to scaffold files"
        );
        return Ok(());
    }

    for spec in &specs {
        write_template(spec, args.overwrite)?;
    }

    println!(
        "[xtask][new-component] generated {} automation files",
        specs.len()
    );
    println!(
        "[xtask][new-component] next steps: update docs stub {}, wire adapters, and replace ignored tests",
        ctx.docs_stub_relative()
    );
    Ok(())
}

fn build_template_specs(
    workspace: &Path,
    template_root: &Path,
    ctx: &TemplateContext,
    audience: TemplateAudience,
) -> Result<Vec<TemplateSpec>> {
    let mut entries = Vec::new();

    if matches!(audience, TemplateAudience::All | TemplateAudience::Material) {
        entries.push(TemplateEntry {
            template: "material.rs.tpl",
            output: format!("crates/rustic-ui-material/src/{}.rs", ctx.component_snake),
            description: "Material Rust module",
        });
        entries.push(TemplateEntry {
            template: "material_test.rs.tpl",
            output: format!(
                "crates/rustic-ui-material/tests/{}_adapters.rs",
                ctx.component_snake
            ),
            description: "Material integration test placeholder",
        });
        entries.push(TemplateEntry {
            template: "typescript_adapter.tsx.tpl",
            output: format!(
                "packages/mui-material/src/{}/RusticAdapter.tsx",
                ctx.component_pascal
            ),
            description: "React/TypeScript adapter telemetry helper",
        });
    }

    if matches!(audience, TemplateAudience::All | TemplateAudience::Headless) {
        entries.push(TemplateEntry {
            template: "headless.rs.tpl",
            output: format!("crates/rustic-ui-headless/src/{}.rs", ctx.component_snake),
            description: "Headless Rust module",
        });
        entries.push(TemplateEntry {
            template: "headless_test.rs.tpl",
            output: format!(
                "crates/rustic-ui-headless/tests/{}_state.rs",
                ctx.component_snake
            ),
            description: "Headless integration test placeholder",
        });
    }

    entries.push(TemplateEntry {
        template: "docs_stub.mdx.tpl",
        output: format!(
            "docs/src/pages/system/components/{}.mdx",
            ctx.component_kebab
        ),
        description: "Docs MDX stub",
    });

    let mut specs = Vec::with_capacity(entries.len());
    for entry in entries {
        let template_path = template_root.join(entry.template);
        let rendered = render_template(&template_path, ctx)?;
        let output = workspace.join(entry.output);
        specs.push(TemplateSpec {
            output,
            rendered,
            description: entry.description.to_string(),
        });
    }

    Ok(specs)
}

fn render_template(path: &Path, ctx: &TemplateContext) -> Result<String> {
    let template = fs::read_to_string(path)
        .with_context(|| format!("failed to read template {}", path.display()))?;
    let mut rendered = template.replace("{{component_pascal}}", &ctx.component_pascal);
    rendered = rendered.replace("{{component_snake}}", &ctx.component_snake);
    rendered = rendered.replace("{{component_kebab}}", &ctx.component_kebab);
    rendered = rendered.replace("{{component_shouty_snake}}", &ctx.component_shouty);
    rendered = rendered.replace("{{automation_id}}", &ctx.automation_id);
    Ok(rendered)
}

fn write_template(spec: &TemplateSpec, overwrite: bool) -> Result<()> {
    if let Some(parent) = spec.output.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory {}", parent.display()))?;
    }

    if spec.output.exists() && !overwrite {
        bail!(
            "refusing to overwrite existing file {}; rerun with --overwrite if this is intentional",
            spec.output.display()
        );
    }

    fs::write(&spec.output, &spec.rendered)
        .with_context(|| format!("failed to write generated file {}", spec.output.display()))?;
    println!(
        "[xtask][new-component] wrote {} ({})",
        spec.output.display(),
        spec.description
    );
    Ok(())
}

struct TemplateSpec {
    output: PathBuf,
    rendered: String,
    description: String,
}

struct TemplateEntry {
    template: &'static str,
    output: String,
    description: &'static str,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum TemplateAudience {
    Material,
    Headless,
    All,
}

struct TemplateContext {
    component_pascal: String,
    component_snake: String,
    component_kebab: String,
    component_shouty: String,
    automation_id: String,
}

impl TemplateContext {
    fn docs_stub_relative(&self) -> String {
        format!(
            "docs/src/pages/system/components/{}.mdx",
            self.component_kebab
        )
    }
}
