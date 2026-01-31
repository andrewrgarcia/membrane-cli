use crate::core::Project;
use crate::memfs;
use crate::global;
use crate::utils::render::render_key_value;
use crate::utils::resolve::resolve_project;
use crate::commands::show_model::ShowContext;

use anyhow::Result;
use colored::Colorize;
use serde_yaml::Value;
use std::cmp::Ordering;
use std::fs;
use std::path::Path;

// ------------------------------------------------------------
// Public entry
// ------------------------------------------------------------

pub fn run(
    project: Option<&str>,
    sort_key: Option<&str>,
    desc: bool,
    printed: bool,
    only: bool,
) -> Result<()> {
    let root = memfs::resolve_workspace_root()?;
    let projects_dir = memfs::projects_dir(&root);

    let index = global::load_global_index();
    let brane_id = index
        .active
        .clone()
        .unwrap_or_else(|| "unknown".into());

    let mut projects = load_projects(&projects_dir)?;

    if let Some(k) = sort_key {
        sort_projects(&mut projects, k, desc);
    }

    if only {
        if let Some(k) = sort_key {
            projects.retain(|(_, p)| p.contains_key(k));
        }
    }

    let ctx = ShowContext {
        brane_root: root.clone(),
        brane_id,
        sort_key: sort_key.map(|s| s.to_string()),
        projects,
    };

    // --- CLI output (colored)
    render_cli(&ctx, project)?;

    // --- Markdown output (clean)
    if printed {
        render_markdown(&ctx)?;
    }

    Ok(())
}

// ------------------------------------------------------------
// List projects
// ------------------------------------------------------------


fn render_cli(ctx: &ShowContext, project: Option<&str>) -> Result<()> {
    let short = ctx.brane_id.chars().take(8).collect::<String>();

    println!(
        "{} {}  [{}]",
        "Active brane:".dimmed(),
        ctx.brane_root.display(),
        short
    );

    match project {
        Some(p) => render_single_cli(ctx, p),
        None => render_list_cli(ctx),
    }
}

fn render_list_cli(ctx: &ShowContext) -> Result<()> {
    let header = match &ctx.sort_key {
        Some(k) => format!("=== Projects (sorted by {}) ===", k),
        None => "=== Projects ===".to_string(),
    };

    println!("{}", header.truecolor(255,105,180).bold());

    for (name, project) in &ctx.projects {
        let id = project
            .get("_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let short_id = id.chars().take(8).collect::<String>();

        if let Some(k) = &ctx.sort_key {
            let val = project
                .get(k)
                .and_then(render_inline_value)
                .unwrap_or("—".into());

            println!(
                "• {:<20} {}: {:<20} {}",
                name.bright_white(),
                k.dimmed(),
                val.dimmed(),
                format!("[{}]", short_id).dimmed()
            );
        } else {
            println!(
                "• {:<20} {}",
                name.bright_white(),
                format!("[{}]", short_id).dimmed()
            );
        }
    }

    Ok(())
}


fn render_single_cli(ctx: &ShowContext, input: &str) -> Result<()> {
    let dir = memfs::projects_dir(&ctx.brane_root);
    let (name, project) = resolve_project(&dir, input)?;

    println!(
        "{}",
        format!("— {} —", name)
            .truecolor(255,105,180)
            .bold()
    );

    for (key, value) in project {
        match value {
            Value::Bool(_)
            | Value::Number(_)
            | Value::String(_)
            | Value::Null => {
                let rendered = serde_yaml::to_string(&value)?.trim().to_string();
                let (k, v) = render_key_value(&key, &rendered);
                println!("{k}: {v}");
            }
            _ => {
                let (k, _) = render_key_value(&key, "");
                println!("{k}:");
                let rendered = serde_yaml::to_string(&value)?;
                for line in rendered.lines() {
                    println!("  {}", line);
                }
            }
        }
    }

    Ok(())
}

fn render_markdown(ctx: &ShowContext) -> Result<()> {
    let mut md = String::new();
    let short = ctx.brane_id.chars().take(8).collect::<String>();

    md.push_str(&format!(
        "> Active brane: {} [{}]\n\n",
        ctx.brane_root.display(),
        short
    ));

    let title = match &ctx.sort_key {
        Some(k) => format!("# Projects (sorted by {})\n\n", k),
        None => "# Projects\n\n".to_string(),
    };

    md.push_str(&title);

    // --- Index
    for (name, project) in &ctx.projects {
        let id = project
            .get("_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let short_id = id.chars().take(8).collect::<String>();

        if let Some(k) = &ctx.sort_key {
            let val = project
                .get(k)
                .and_then(render_inline_value)
                .unwrap_or("—".into());

            md.push_str(&format!(
                "• {:<20} {}: {:<20} [{}]\n",
                name, k, val, short_id
            ));
        } else {
            md.push_str(&format!(
                "• {:<20} [{}]\n",
                name, short_id
            ));
        }
    }

    md.push_str("\n---\n\n");

    // --- Full projects
    for (name, project) in &ctx.projects {
        md.push_str(&format!("## {}\n", name));

        for (key, value) in project {
            match value {
                Value::Bool(_)
                | Value::Number(_)
                | Value::String(_)
                | Value::Null => {
                    md.push_str(&format!(
                        "{}: {}\n",
                        key,
                        serde_yaml::to_string(value)?.trim()
                    ));
                }
                _ => {
                    md.push_str(&format!("{}:\n", key));
                    let rendered = serde_yaml::to_string(value)?;
                    for line in rendered.lines() {
                        md.push_str(&format!("  {}\n", line));
                    }
                }
            }
        }

        md.push('\n');
    }

    let filename = format!("BRANE_{}.md", short);
    fs::write(&filename, md)?;

    println!("✔ wrote {}", filename);

    Ok(())
}


// ------------------------------------------------------------
// Helpers
// ------------------------------------------------------------

fn render_inline_value(value: &Value) -> Option<String> {
    match value {
        Value::Bool(_)
        | Value::Number(_)
        | Value::String(_)
        | Value::Null => {
            Some(
                serde_yaml::to_string(value)
                    .ok()?
                    .trim()
                    .to_string()
            )
        }
        _ => None,
    }
}

fn load_projects(dir: &Path) -> Result<Vec<(String, Project)>> {
    let mut out = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
            continue;
        }

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap()
            .to_string();

        let content = fs::read_to_string(&path)?;
        let mut project: Project = serde_yaml::from_str(&content)?;

        if !project.contains_key("_id") {
            let id = uuid::Uuid::new_v4().to_string();
            project.insert("_id".into(), Value::String(id));
            fs::write(&path, serde_yaml::to_string(&project)?)?;
        }

        out.push((name, project));
    }

    Ok(out)
}

fn sort_projects(
    projects: &mut Vec<(String, Project)>,
    key: &str,
    desc: bool,
) {
    projects.sort_by(|a, b| {
        let va = a.1.get(key);
        let vb = b.1.get(key);
        let ord = compare_yaml_values(va, vb);
        if desc { ord.reverse() } else { ord }
    });
}

fn compare_yaml_values(a: Option<&Value>, b: Option<&Value>) -> Ordering {
    match (a, b) {
        (None, None) => Ordering::Equal,
        (None, Some(_)) => Ordering::Greater,
        (Some(_), None) => Ordering::Less,
        (Some(Value::Number(a)), Some(Value::Number(b))) =>
            a.as_f64().partial_cmp(&b.as_f64()).unwrap_or(Ordering::Equal),
        (Some(Value::String(a)), Some(Value::String(b))) =>
            a.cmp(b),
        (Some(a), Some(b)) =>
            format!("{a:?}").cmp(&format!("{b:?}")),
    }
}
