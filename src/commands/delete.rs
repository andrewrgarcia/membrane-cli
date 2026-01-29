use anyhow::Result;
use std::{fs, io::{self, Write}};

use crate::memfs;

pub fn run(project: &str) -> Result<()> {
    let root = memfs::find_membrane_root()?;
    let path = memfs::projects_dir(&root).join(format!("{project}.yaml"));

    if !path.exists() {
        anyhow::bail!("Project not found: {project}");
    }

    // Warning
    println!("⚠️  You are about to permanently delete the project:");
    println!("    {project}");
    println!();
    println!("This action cannot be undone.");
    println!("Type the project name to confirm deletion:");

    // Prompt
    print!("> ");
    io::stdout().flush()?; // ensure prompt is shown

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input != project {
        println!("Aborted. Project was not deleted.");
        return Ok(());
    }

    // Delete
    fs::remove_file(&path)?;
    println!("Project `{project}` deleted.");

    Ok(())
}
