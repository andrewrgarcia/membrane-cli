use anyhow::Result;
use std::fs;

use crate::core::Project;
use crate::memfs;
use crate::utils::time::now_iso;

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

    if data.remove(key).is_none() {
        anyhow::bail!("Key `{}` not found in project `{}`", key, project);
    }

    // update timestamp
    data.insert(
        "_updated".to_string(),
        serde_yaml::Value::String(now_iso()),
    );

    fs::write(&path, serde_yaml::to_string(&data)?)?;

    println!("✔ removed key `{}` from `{}`", key, project);
    Ok(())
}
