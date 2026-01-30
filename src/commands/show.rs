use crate::core::Project;
use crate::memfs;
use crate::utils::render::render_key_value;

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
) -> Result<()> {
    let root = memfs::resolve_workspace_root()?;
    let projects_dir = memfs::projects_dir(&root);

    match project {
        Some(name) => show_single(&projects_dir, name),
        None => show_all(&projects_dir, sort_key, desc),
    }
}

// ------------------------------------------------------------
// List projects (optionally sorted)
// ------------------------------------------------------------

fn show_all(
    dir: &Path,
    sort_key: Option<&str>,
    desc: bool,
) -> Result<()> {
    let mut projects = load_projects(dir)?;

    if let Some(key) = sort_key {
        sort_projects(&mut projects, key, desc);
    }

    let show_key = sort_key.and_then(|k| {
        projects.iter().any(|(_, p)| p.contains_key(k)).then_some(k)
    });

    if let Some(k) = show_key {
        println!(
            "{}",
            format!("=== Projects (sorted by {}) ===", k)
                .truecolor(255, 105, 180)
                .bold()
        );
    } else {
        println!(
            "{}",
            "=== Projects ==="
                .truecolor(255, 105, 180)
                .bold()
        );
    }

    for (name, project) in projects {
        let id_suffix = project
            .get("_id")
            .and_then(|v| v.as_str())
            .map(|s| format!("[{}]", &s[..8]).dimmed())
            .unwrap_or_else(|| "".normal());

        // ── SORTED CASE ───────────────────────────────
        if let Some(k) = show_key {
            let val = project
                .get(k)
                .and_then(render_inline_value)
                .unwrap_or_else(|| "—".into());

            println!(
                "• {:<20} {}: {:<20} {}",
                name,
                k.dimmed(),
                val.dimmed(),
                id_suffix
            );
            continue;
        }

        // ── UNSORTED CASE ─────────────────────────────
        println!(
            "• {:<20} {}",
            name,
            id_suffix
        );
    }

    Ok(())
}


fn render_inline_value(value: &serde_yaml::Value) -> Option<String> {
    match value {
        serde_yaml::Value::Bool(_)
        | serde_yaml::Value::Number(_)
        | serde_yaml::Value::String(_)
        | serde_yaml::Value::Null => {
            Some(
                serde_yaml::to_string(value)
                    .ok()?
                    .trim()
                    .to_string()
            )
        }
        _ => None, // sequences / maps are too noisy inline
    }
}

// ------------------------------------------------------------
// Show single project
// ------------------------------------------------------------



use crate::utils::resolve::resolve_project;

fn show_single(dir: &Path, input: &str) -> Result<()> {
    let (name, project) = resolve_project(dir, input)?;

    println!(
        "{}",
        format!("— {} —", name)
            .truecolor(255, 105, 180)
            .bold()
    );

    for (key, value) in project {
        match value {
            Value::Bool(_)
            | Value::Number(_)
            | Value::String(_)
            | Value::Null => {
                let rendered = serde_yaml::to_string(&value)?
                    .trim()
                    .to_string();
                let (k, v) = render_key_value(&key, &rendered);
                println!("{k}: {v}");
            }

            // sequences and mappings
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


// ------------------------------------------------------------
// Load + sort helpers
// ------------------------------------------------------------

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

        // BACKFILL ID IF MISSING
        if !project.contains_key("_id") {
            let new_id = uuid::Uuid::new_v4().to_string();
            project.insert("_id".to_string(), Value::String(new_id));

            // persist immediately
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
            a.as_f64()
                .partial_cmp(&b.as_f64())
                .unwrap_or(Ordering::Equal),

        (Some(Value::String(a)), Some(Value::String(b))) =>
            a.cmp(b),

        (Some(a), Some(b)) =>
            format!("{a:?}").cmp(&format!("{b:?}")),
    }
}

