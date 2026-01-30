use anyhow::{Result, Context};
use std::{fs, path::PathBuf};
use uuid::Uuid;

pub fn find_membrane_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir()?;

    loop {
        if dir.join(".membrane").exists() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }

    anyhow::bail!("Not inside a Membrane workspace. Run `mem init` first.")
}

pub fn projects_dir(root: &PathBuf) -> PathBuf {
    root.join(".membrane").join("projects")
}

pub fn init_membrane() -> Result<()> {
    let root = std::env::current_dir()?;
    let membrane = root.join(".membrane");
    let projects = membrane.join("projects");

    fs::create_dir_all(&projects)
        .with_context(|| "Failed to create .membrane/projects")?;

    // --- membrane ID (stable)
    let id_path = membrane.join("id");
    if !id_path.exists() {
        fs::write(&id_path, Uuid::new_v4().to_string())?;
    }

    fs::write(membrane.join("config.yaml"), "version: 0.2\n")?;

    println!("Initialized Membrane in {}", root.display());
    Ok(())
}
