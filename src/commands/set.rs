use anyhow::Result;
use std::fs;

use crate::core::Project;
use crate::memfs;
use crate::utils::{
    parse::parse_scalar,
    time::now_iso,
    input::read_multiline,
};

pub fn run(project: &str, key: &str, value: Option<&str>) -> Result<()> {
    let root = memfs::find_membrane_root()?;
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
    data.insert(
        "_updated".to_string(),
        serde_yaml::Value::String(now_iso()),
    );

    fs::write(&path, serde_yaml::to_string(&data)?)?;
    println!("Set `{key}` on `{project}`");

    Ok(())
}
