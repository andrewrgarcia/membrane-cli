use anyhow::Result;
use std::{collections::HashMap, fs};
use walkdir::WalkDir;

use crate::core::Project;

// ---------- helpers ----------

fn normalize(key: &str) -> String {
    key.to_lowercase()
        .replace('_', "")
        .replace('-', "")
}

// ---------- public API ----------

pub fn sweep_keys(projects_dir: &std::path::Path) -> Result<HashMap<String, usize>> {
    let mut counts: HashMap<String, usize> = HashMap::new();

    for project in load_projects(projects_dir)? {
        for key in project.keys() {
            *counts.entry(key.clone()).or_insert(0) += 1;
        }
    }

    Ok(counts)
}

pub fn sweep_similar_keys(
    projects_dir: &std::path::Path,
) -> Result<HashMap<String, Vec<String>>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for project in load_projects(projects_dir)? {
        for key in project.keys() {
            let norm = normalize(key);
            groups.entry(norm).or_default().push(key.clone());
        }
    }

    Ok(groups)
}

// ---------- internal ----------

fn load_projects(projects_dir: &std::path::Path) -> Result<Vec<Project>> {
    let mut projects = Vec::new();

    for entry in WalkDir::new(projects_dir)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.path().extension().and_then(|s| s.to_str()) == Some("yaml") {
            let content = fs::read_to_string(entry.path())?;
            let project: Project = serde_yaml::from_str(&content)?;
            projects.push(project);
        }
    }

    Ok(projects)
}
