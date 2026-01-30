use anyhow::Result;
use std::fs;

use crate::core::Project;
use crate::memfs;
use crate::utils::time::now_iso;

pub fn run(old: &str, new: &str, project_filter: Option<&str>) -> Result<()> {
    if old == new {
        anyhow::bail!("Old key and new key are identical");
    }

    let root = memfs::find_membrane_root()?;
    let projects_dir = memfs::projects_dir(&root);

    println!("Renaming key `{}` → `{}`", old, new);
    if let Some(p) = project_filter {
        println!("Scope: project `{}`", p);
    }
    println!();

    let mut updated = 0;

    for entry in fs::read_dir(&projects_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
            continue;
        }

        let name = path_stem(&path);

        // --- apply project filter if present
        if let Some(filter) = project_filter {
            if name != filter {
                continue;
            }
        }

        let content = fs::read_to_string(&path)?;
        let mut project: Project = serde_yaml::from_str(&content)?;

        if !project.contains_key(old) {
            println!("– {} (key not present)", name);
            continue;
        }

        if project.contains_key(new) {
            anyhow::bail!(
                "Project `{}` already contains key `{}` — aborting",
                name,
                new
            );
        }

        let value = project.remove(old).unwrap();
        project.insert(new.to_string(), value);

        project.insert(
            "_updated".to_string(),
            serde_yaml::Value::String(now_iso()),
        );

        fs::write(&path, serde_yaml::to_string(&project)?)?;
        println!("✔ {}", name);
        updated += 1;
    }

    if updated == 0 {
        println!("\nNo projects were updated.");
    } else {
        println!("\nDone. Updated {} project(s).", updated);
    }

    Ok(())
}

fn path_stem(path: &std::path::Path) -> String {
    path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string()
}
