use anyhow::Result;
use std::fs;
use std::path::Path;
use serde_yaml::Value;

use crate::core::Project;

pub fn resolve_project(
    dir: &Path,
    input: &str,
) -> Result<(String, Project)> {
    let mut exact = None;
    let mut id_matches = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) != Some("yaml") {
            continue;
        }

        let name = path.file_stem().unwrap().to_string_lossy().to_string();
        let content = fs::read_to_string(&path)?;
        let project: Project = serde_yaml::from_str(&content)?;

        // 1. Exact name match
        if name == input {
            exact = Some((name, project));
            break;
        }

        // 2. ID prefix match
        if let Some(Value::String(id)) = project.get("_id") {
            if id.starts_with(input) {
                id_matches.push((name, project));
            }
        }
    }

    if let Some(hit) = exact {
        return Ok(hit);
    }

    match id_matches.len() {
        0 => anyhow::bail!("No project matches '{}'", input),
        1 => Ok(id_matches.remove(0)),
        _ => {
            let ids: Vec<String> = id_matches
                .iter()
                .filter_map(|(_, p)| p.get("_id"))
                .filter_map(|v| v.as_str())
                .map(|s| s[..8].to_string())
                .collect();

            anyhow::bail!("Ambiguous ID prefix '{}': {:?}", input, ids)
        }
    }
}
