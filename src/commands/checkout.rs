use anyhow::Result;
use crate::global;

pub fn run(target: &str) -> Result<()> {
    let mut index = global::load_global_index();

    let matches: Vec<_> = index
        .workspaces
        .iter()
        .filter(|w| w.id.starts_with(target))
        .collect();

    match matches.len() {
        0 => anyhow::bail!("No workspace matches '{}'", target),
        1 => {
            index.active = Some(matches[0].id.clone());
            global::save_global_index(&index)?;
            println!(
                "✔ Switched to membrane {}",
                &matches[0].id[..8]
            );
            Ok(())
        }
        _ => {
            let ids: Vec<_> = matches.iter().map(|w| &w.id[..8]).collect();
            anyhow::bail!("Ambiguous prefix '{}': {:?}", target, ids)
        }
    }
}
