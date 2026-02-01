use anyhow::Result;
use std::fs;

use crate::core::Project;
use crate::memfs;
use crate::utils::project_writer::{canonicalize_project, write_project};


/// Remove a key from a project
pub fn run(project: &str, key: &str) -> Result<()> {
    if key.starts_with('_') {
        anyhow::bail!("Refusing to delete reserved metadata key `{}`", key);
    }

    let root = memfs::resolve_workspace_root()?;
    let path = memfs::projects_dir(&root).join(format!("{project}.yaml"));

    if !path.exists() {
        anyhow::bail!("Project not found: {project}");
    }

    let content = fs::read_to_string(&path)?;
    let mut data: Project = serde_yaml::from_str(&content)?;

    if data.shift_remove(key).is_none() {
        anyhow::bail!("Key `{}` not found in project `{}`", key, project);
    }

    // update timestamp
    let ordered = canonicalize_project(data, project)?;
    write_project(&path, ordered)?;


    println!("✔ removed key `{}` from `{}`", key, project);
    Ok(())
}
