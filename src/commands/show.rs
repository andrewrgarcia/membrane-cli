use crate::core::Project;
use crate::memfs;
use crate::global;
use crate::utils::render::render_key_value;
use crate::utils::resolve::resolve_project;

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

    let mut out = Vec::<String>::new();

    // ── Active brane header ────────────────────────────────
    let index = global::load_global_index();

    if let Some(active_id) = &index.active {
        let short = active_id.get(..8).unwrap_or(active_id);
        out.push(format!(
            "Active brane: {}  [{}]",
            root.display(),
            short
        ));
    }


    match project {
        Some(name) => show_single(&projects_dir, name, &mut out)?,
        None => show_all(&projects_dir, sort_key, desc, only, &mut out)?,
    }

    // ── Emit ───────────────────────────────────────────────
    for line in &out {
        println!("{}", line);
    }

    if printed {
        fs::write("SHOW.md", out.join("\n"))?;
    }

    Ok(())
}

// ------------------------------------------------------------
// List projects
// ------------------------------------------------------------

fn show_all(
    dir: &Path,
    sort_key: Option<&str>,
    desc: bool,
    only: bool,
    out: &mut Vec<String>,
) -> Result<()> {
    let mut projects = load_projects(dir)?;

    if let Some(key) = sort_key {
        sort_projects(&mut projects, key, desc);
    }

    let key_exists = sort_key.and_then(|k| {
        projects.iter().any(|(_, p)| p.contains_key(k)).then_some(k)
    });

    // ── HEADER ────────────────────────────────────────────────
    let header = match key_exists {
        Some(k) => format!(
            "=== Projects (sorted by {}) ===",
            k.dimmed()
        )
        .truecolor(255, 105, 180)
        .bold()
        .to_string(),

        None => "=== Projects ==="
            .truecolor(255, 105, 180)
            .bold()
            .to_string(),
    };

    out.push(header);

    // ── BODY ──────────────────────────────────────────────────
    for (name, project) in projects {
        // --only filtering
        if let (true, Some(k)) = (only, key_exists) {
            if !project.contains_key(k) {
                continue;
            }
        }

        let id = project
            .get("_id")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let short_id = id.get(..8).unwrap_or(id);

        let name_col = name.bright_white();
        let id_col = format!("[{}]", short_id).dimmed();

        if let Some(k) = key_exists {
            let val = project
                .get(k)
                .and_then(render_inline_value)
                .unwrap_or_else(|| "—".into());

            out.push(format!(
                "• {:<20} {}: {:<20} {}",
                name_col,
                k.dimmed(),
                val.dimmed(),
                id_col
            ));
        } else {
            out.push(format!(
                "• {:<20} {}",
                name_col,
                id_col
            ));
        }
    }

    Ok(())
}


// ------------------------------------------------------------
// Single project
// ------------------------------------------------------------

fn show_single(dir: &Path, input: &str, out: &mut Vec<String>) -> Result<()> {
    let (name, project) = resolve_project(dir, input)?;

    out.push(format!("— {} —", name));

    for (key, value) in project {
        match value {
            Value::Bool(_)
            | Value::Number(_)
            | Value::String(_)
            | Value::Null => {
                let rendered = serde_yaml::to_string(&value)?.trim().to_string();
                let (k, v) = render_key_value(&key, &rendered);
                out.push(format!("{k}: {v}"));
            }
            _ => {
                let (k, _) = render_key_value(&key, "");
                out.push(format!("{k}:"));
                let rendered = serde_yaml::to_string(&value)?;
                for line in rendered.lines() {
                    out.push(format!("  {}", line));
                }
            }
        }
    }

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
