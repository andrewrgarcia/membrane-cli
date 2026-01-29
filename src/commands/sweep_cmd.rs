use anyhow::Result;
use std::collections::HashSet;

use crate::{memfs, sweep};
use crate::utils::table::render_table;

pub fn run(similar: bool) -> Result<()> {
    let root = memfs::find_membrane_root()?;
    let dir = memfs::projects_dir(&root);

    if similar {
        let groups = sweep::sweep_similar_keys(&dir)?;
        println!("Possible duplicate keys:\n");

        for (_norm, keys) in groups {
            let uniq: HashSet<_> = keys.iter().collect();
            if uniq.len() > 1 {
                let line = uniq.into_iter().cloned().collect::<Vec<_>>().join(", ");
                println!("{line}");
            }
        }
    } else {
        let counts = sweep::sweep_keys(&dir)?;

        // Convert to rows
        let mut rows: Vec<Vec<String>> = counts
            .into_iter()
            .map(|(k, c)| vec![k, c.to_string()])
            .collect();

        // Optional: sort by count desc, then key
        rows.sort_by(|a, b| {
            b[1].cmp(&a[1]).then_with(|| a[0].cmp(&b[0]))
        });

        render_table(
            "Key Usage",
            &["Key", "Count"],
            rows,
            None,
        );
    }

    Ok(())
}
