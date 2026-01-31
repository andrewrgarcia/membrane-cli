use anyhow::Result;
use crate::global;

pub fn run() -> Result<()> {
    let mut index = global::load_global_index();

    let before = index.workspaces.len();

    // Keep only existing roots
    index.workspaces.retain(|w| w.root.exists());

    let after = index.workspaces.len();
    let removed = before - after;

    // Fix active if needed
    if let Some(active) = &index.active {
        if !index.workspaces.iter().any(|w| &w.id == active) {
            index.active = None;
            println!("Active brane was orphaned and has been unset.");
        }
    }

    global::save_global_index(&index)?;

    println!("Pruned {} orphan brane(s).", removed);

    Ok(())
}
