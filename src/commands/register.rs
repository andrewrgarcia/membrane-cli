use anyhow::Result;
use std::fs;
use uuid::Uuid;

use crate::memfs;
use crate::global;

pub fn run() -> Result<()> {
    // Must be inside a membrane workspace
    let root = memfs::resolve_workspace_root()?;
    let membrane_dir = root.join(".membrane");
    let id_path = membrane_dir.join("id");

    // Ensure membrane dir exists (paranoia-safe)
    fs::create_dir_all(&membrane_dir)?;

    // Read or create membrane ID
    let id = match fs::read_to_string(&id_path) {
        Ok(s) if !s.trim().is_empty() => s.trim().to_string(),
        _ => {
            let new_id = Uuid::new_v4().to_string();
            fs::write(&id_path, &new_id)?;
            new_id
        }
    };

    // Register globally
    global::register_workspace(&id, &root)?;

    println!(
        "✔ registered membrane {} at {}",
        &id[..8],
        root.display()
    );

    Ok(())
}
