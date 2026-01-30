use anyhow::{Result, Context};
use std::{fs, path::PathBuf};
use uuid::Uuid;
use crate::global;

/// Find membrane root by walking up from CWD
pub fn find_membrane_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir()?;

    loop {
        if dir.join(".membrane").is_dir() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }

    anyhow::bail!("Not inside a Membrane workspace. Run `me init` first.")
}


/// Resolve the workspace root.
/// Priority:
/// 1. Globally active workspace (via `me checkout`)
/// 2. Nearest `.membrane` from cwd (legacy / fallback)
pub fn resolve_workspace_root() -> Result<PathBuf> {
    if let Ok(root) = global::active_workspace_root() {
        return Ok(root);
    }

    find_membrane_root()
}


pub fn projects_dir(root: &PathBuf) -> PathBuf {
    root.join(".membrane").join("projects")
}

/// Initialize a local membrane workspace
pub fn init_membrane() -> Result<()> {
    let root = std::env::current_dir()?;
    let membrane = root.join(".membrane");
    let projects = membrane.join("projects");

    fs::create_dir_all(&projects)
        .with_context(|| "Failed to create .membrane/projects")?;

    // --- membrane ID (stable)
    let id_path = membrane.join("id");
    let id = if id_path.exists() {
        fs::read_to_string(&id_path)?.trim().to_string()
    } else {
        let new_id = Uuid::new_v4().to_string();
        fs::write(&id_path, &new_id)?;
        new_id
    };

    // 🔑 REGISTER THIS WORKSPACE GLOBALLY
    global::register_workspace(&id, &root)?;

    fs::write(membrane.join("config.yaml"), "version: 0.2\n")?;

    println!("Initialized Membrane in {}", root.display());
    Ok(())
}
