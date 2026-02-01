use anyhow::Result;
use std::fs;

use crate::core::Project;
use crate::memfs;
use crate::utils::{
    parse::parse_scalar,
    input::read_multiline,
    project_writer::{materialize_project, write_project}
};

pub fn run(project: &str, key: &str, value: Option<&str>) -> Result<()> {
    // guard reserved keys EARLY
    if key.starts_with('_') {
        anyhow::bail!("Keys starting with '_' are reserved metadata keys.");
    }

    let root = memfs::resolve_workspace_root()?;
    let path = memfs::projects_dir(&root).join(format!("{project}.yaml"));

    if !path.exists() {
        anyhow::bail!("Project not found: {project}");
    }

    let content = fs::read_to_string(&path)?;
    let mut data: Project = serde_yaml::from_str(&content)?;

    // --- Determine value
    let yaml_value = match value {
        Some(v) => parse_scalar(v),
        None => {
            let input = read_multiline(&format!(
                "✏️  Enter value for key `{}`:",
                key
            ))?;
            serde_yaml::Value::String(input)
        }
    };

    data.insert(key.to_string(), yaml_value);
    let ordered = materialize_project(data, project)?;
    write_project(&path, ordered)?;


    println!("Set `{key}` on `{project}`");

    Ok(())
}
