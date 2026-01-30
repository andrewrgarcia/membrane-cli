use anyhow::Result;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

use serde_yaml::Value;

use crate::core::Project;
use crate::memfs;
use crate::utils::time::now_iso;

/// Entry point
pub fn run(file: Option<&str>, as_name: Option<&str>) -> Result<()> {
    match file {
        Some(path) => push_from_file(path, as_name),
        None => push_from_stdin(as_name),
    }
}

// ------------------------------
// File-based push
// ------------------------------

fn push_from_file(file: &str, as_name: Option<&str>) -> Result<()> {
    let path = Path::new(file);

    if !path.exists() {
        anyhow::bail!("File not found: {}", file);
    }

    let content = fs::read_to_string(path)?;
    let data: Project = serde_yaml::from_str(&content)
        .map_err(|_| anyhow::anyhow!("File must be a YAML mapping (key-value pairs)"))?;

    let project_name = resolve_project_name(
        as_name,
        &data,
        Some(path),
    )?;

    push_project(data, &project_name, file)
}

// ------------------------------
// Interactive stdin push
// ------------------------------

fn push_from_stdin(as_name: Option<&str>) -> Result<()> {
    println!("✍️  Write your project content below.");
    println!("↪ Use YAML-style key/value pairs.");
    println!("↪ Finish with Ctrl+D (Linux/macOS) or Ctrl+Z then Enter (Windows).");
    println!("↪ Ctrl+C to cancel.\n");

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    if buffer.trim().is_empty() {
        anyhow::bail!("No content provided.");
    }

    let data: Project = serde_yaml::from_str(&buffer)
        .map_err(|_| anyhow::anyhow!("Input must be a YAML mapping (key-value pairs)"))?;

    let project_name = resolve_project_name(
        as_name,
        &data,
        None,
    )?;

    push_project(data, &project_name, "stdin")
}

// ------------------------------
// Core push logic (single source of truth)
// ------------------------------

fn push_project(
    mut data: Project,
    project_name: &str,
    source: &str,
) -> Result<()> {
    let root = memfs::find_membrane_root()?;
    let dest = memfs::projects_dir(&root)
        .join(format!("{}.yaml", project_name));

    if dest.exists() {
        anyhow::bail!(
            "Project `{}` already exists. Use --as to choose a different name.",
            project_name
        );
    }

    if !data.contains_key("_id") {
        data.insert(
            "_id".to_string(),
            Value::String(uuid::Uuid::new_v4().to_string()),
        );
    }

    // --- inject metadata
    let now = now_iso();
    data.insert("_created".to_string(), Value::String(now.clone()));
    data.insert("_updated".to_string(), Value::String(now));

    fs::write(&dest, serde_yaml::to_string(&data)?)?;

    println!("✔ pushed {} → project `{}`", source, project_name);
    Ok(())
}

// ------------------------------
// Helpers
// ------------------------------

fn resolve_project_name(
    as_name: Option<&str>,
    data: &Project,
    file_path: Option<&Path>,
) -> Result<String> {
    if let Some(name) = as_name {
        return Ok(slugify(name));
    }

    if let Some(Value::String(name)) = data.get("name") {
        return Ok(slugify(name));
    }

    if let Some(path) = file_path {
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            return Ok(slugify(stem));
        }
    }

    anyhow::bail!("Project name not specified (use `name:` or --as)")
}

fn slugify(input: &str) -> String {
    input
        .trim()
        .to_lowercase()
        .replace(' ', "-")
        .replace('_', "-")
}
