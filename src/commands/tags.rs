use std::collections::BTreeMap;

use crate::store::load;

pub fn run() -> Result<(), String> {
    let store = load()?;
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();

    for task in store.tasks.iter().filter(|t| !t.is_trash) {
        for tag in &task.tags {
            *counts.entry(tag.clone()).or_insert(0) += 1;
        }
    }

    if counts.is_empty() {
        println!("Aucun tag.");
        return Ok(());
    }

    println!("=== Tags ===");
    println!("{:<28} {}", "TAG", "TÂCHES");
    println!("{}", "-".repeat(40));
    for (tag, count) in counts {
        println!("{:<28} {}", tag, count);
    }

    Ok(())
}
