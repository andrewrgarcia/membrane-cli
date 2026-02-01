use std::fs;
use std::path::Path;
use anyhow::Result;

use indexmap::IndexMap;
use serde_yaml::Value;
use crate::core::Project;
use crate::utils::time::now_iso;

// ---- single source of truth
const RESERVED_KEYS: &[&str] = &[
    "_id",
    "_created",
    "_updated",
];


// ------------------------------------------------------------
// WRITE BOUNDARY
// ------------------------------------------------------------

pub fn write_project(path: &Path, project: Project) -> Result<()> {
    let yaml = serde_yaml::to_string(&project)?;
    fs::write(path, yaml)?;
    Ok(())
}


// ------------------------------------------------------------
// CANONICALIZER (STRUCTURE AUTHORITY)
// ------------------------------------------------------------

pub fn canonicalize_project(
    mut data: Project,
    project_name: &str,
) -> Result<Project> {

    // 🔥 Strip rogue metadata instead of crashing
    let rogue_keys: Vec<String> = data
        .keys()
        .filter(|k| k.starts_with('_') && !RESERVED_KEYS.contains(&k.as_str()))
        .cloned()
        .collect();

    for k in rogue_keys {
        data.shift_remove(&k);
    }

    let mut ordered: Project = IndexMap::new();

    let now = now_iso();

    let id = data
        .shift_remove("_id")
        .unwrap_or(Value::String(uuid::Uuid::new_v4().to_string()));

    let created = data
        .shift_remove("_created")
        .unwrap_or(Value::String(now.clone()));

    // ---- PIN METADATA
    ordered.insert("_id".into(), id);
    ordered.insert("name".into(), Value::String(project_name.to_string()));
    ordered.insert("_created".into(), created);
    ordered.insert("_updated".into(), Value::String(now));

    // ---- USER KEYS (preserve order)
    for (k, v) in data {
        if !k.starts_with('_') && k != "name" {
            ordered.insert(k, v);
        }
    }

    Ok(ordered)
}
