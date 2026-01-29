use anyhow::Result;
use std::{fs, collections::HashMap};
use serde_yaml::Value;

use crate::memfs;
use crate::utils::time::now_iso;

pub fn run(name: &str) -> Result<()> {
    let root = memfs::find_membrane_root()?;
    let path = memfs::projects_dir(&root).join(format!("{name}.yaml"));

    if path.exists() {
        anyhow::bail!("Project already exists");
    }

    let now = now_iso();

    let mut project: HashMap<String, Value> = HashMap::new();
    project.insert("name".to_string(), Value::String(name.to_string()));
    project.insert("_created".to_string(), Value::String(now.clone()));
    project.insert("_updated".to_string(), Value::String(now));

    fs::write(path, serde_yaml::to_string(&project)?)?;
    Ok(())
}
