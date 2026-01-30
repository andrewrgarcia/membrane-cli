use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::utils::time::now_iso;

#[derive(Debug, Serialize, Deserialize)]
pub struct GlobalIndex {
    pub active: Option<String>,
    pub workspaces: Vec<WorkspaceEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceEntry {
    pub id: String,
    pub root: PathBuf,
    pub last_seen: String,
}

/// ~/.membrane/global.yaml
pub fn global_index_path() -> PathBuf {
    dirs::home_dir()
        .expect("Home directory not found")
        .join(".membrane")
        .join("global.yaml")
}

pub fn load_global_index() -> GlobalIndex {
    let path = global_index_path();

    if !path.exists() {
        return GlobalIndex {
            active: None,
            workspaces: Vec::new(),
        };
    }

    serde_yaml::from_str(&fs::read_to_string(path).unwrap())
        .unwrap_or(GlobalIndex {
            active: None,
            workspaces: Vec::new(),
        })
}

pub fn save_global_index(index: &GlobalIndex) -> Result<()> {
    let path = global_index_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_yaml::to_string(index)?)?;
    Ok(())
}

/// Register or refresh a workspace
pub fn register_workspace(id: &str, root: &Path) -> Result<()> {
    let mut index = load_global_index();
    let now = now_iso();

    if let Some(w) = index.workspaces.iter_mut().find(|w| w.id == id) {
        w.last_seen = now;
        w.root = root.to_path_buf();
    } else {
        index.workspaces.push(WorkspaceEntry {
            id: id.to_string(),
            root: root.to_path_buf(),
            last_seen: now,
        });
    }

    if index.active.is_none() {
        index.active = Some(id.to_string());
    }

    save_global_index(&index)
}

/// Resolve active workspace root
#[allow(dead_code)]
pub fn active_workspace_root() -> Result<PathBuf> {
    let index = load_global_index();

    let active_id = index
        .active
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No active workspace. Run `me brane` or `me checkout`."))
        ?;

    let ws = index
        .workspaces
        .iter()
        .find(|w| &w.id == active_id)
        .ok_or_else(|| anyhow::anyhow!("Active workspace not found"))?;

    Ok(ws.root.clone())
}
