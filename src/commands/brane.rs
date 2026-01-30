use anyhow::Result;
use colored::Colorize;
use walkdir::WalkDir;
use std::fs;

use crate::global;

pub fn run() -> Result<()> {
    // 1. Scan filesystem and register any membranes found
    let home = dirs::home_dir().expect("Home directory not found");

    for entry in WalkDir::new(&home)
        .max_depth(6)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name() == "id" {
            let path = entry.path();

            // only `.membrane/id`
            if path.parent().map(|p| p.ends_with(".membrane")).unwrap_or(false) {
                let membrane_dir = path.parent().unwrap();
                let root = membrane_dir.parent().unwrap();

                if let Ok(id) = fs::read_to_string(path) {
                    let id = id.trim();

                    // HARD GUARD: never register empty IDs
                    if !id.is_empty() {
                        global::register_workspace(id, root)?;
                    }
                }
            }
        }
    }

    // 2. Reload global index
    let index = global::load_global_index();

    println!("{}", "=== Branes ===".bold().truecolor(255, 105, 180));

    // 3. Display safely
    for w in &index.workspaces {
        // GUARD AGAINST CORRUPT ENTRIES
        if w.id.trim().is_empty() {
            continue;
        }

        let marker = if index.active.as_deref() == Some(&w.id) {
            "*"
        } else {
            " "
        };

        // SAFE SHORT ID (NO PANIC POSSIBLE)
        let short_id = w.id.chars().take(8).collect::<String>();

        println!(
            "{} {:<10} {}",
            marker,
            short_id,
            w.root.display().to_string().dimmed()
        );
    }

    Ok(())
}
