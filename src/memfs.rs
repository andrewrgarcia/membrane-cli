use anyhow::{Result, Context};
use std::{fs, path::PathBuf};

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

    fs::write(membrane.join("config.yaml"), "version: 0.1\n")?;

    println!("Initialized Membrane in {}", root.display());
    Ok(())
}
