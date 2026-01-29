use anyhow::Result;
use std::fs;

use crate::core::Project;
use crate::memfs;
use crate::utils::render::render_key_value;
use colored::Colorize;

pub fn run(project: Option<&str>) -> Result<()> {
    match project {
        Some(name) => show_project(name),
        None => list_projects(),
    }
}

// ---------- internal ----------

fn list_projects() -> Result<()> {
    let root = memfs::find_membrane_root()?;
    let mut projects = Vec::new();

    for entry in fs::read_dir(memfs::projects_dir(&root))? {
        let entry = entry?;
        if let Some(name) = entry.path().file_stem() {
            projects.push(name.to_string_lossy().to_string());
        }
    }

    projects.sort();

    println!("{}", "=== Projects ===".bright_purple().bold());

    for name in projects {
        println!("• {}", name);
    }

    Ok(())
}


fn show_project(name: &str) -> Result<()> {
    let root = memfs::find_membrane_root()?;
    let path = memfs::projects_dir(&root).join(format!("{name}.yaml"));

    if !path.exists() {
        anyhow::bail!("Project not found: {name}");
    }

    let content = fs::read_to_string(&path)?;
    let project: Project = serde_yaml::from_str(&content)?;

    println!("— {name} —");

    for (key, value) in project {
        let rendered_val = serde_yaml::to_string(&value)?.trim().to_string();
        let (k, v) = render_key_value(&key, &rendered_val);
        println!("{k}: {v}");
    }

    Ok(())
}
