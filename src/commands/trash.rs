use crate::store::{load, save};

pub fn run(restore: Option<String>, clean_all: bool) -> Result<(), String> {
    let mut store = load()?;

    let trashed: Vec<_> = store.tasks.iter().filter(|t| t.is_trash).collect();

    if let Some(id) = restore {
        let task = store
            .tasks
            .iter_mut()
            .find(|t| t.id == id && t.is_trash)
            .ok_or_else(|| format!("Task {id} non trouvée dans la corbeille."))?;
        task.is_trash = false;
        save(&store)?;
        println!("Task {id} restaurée.");
        return Ok(());
    }

    if clean_all {
        let count = trashed.len();
        if count == 0 {
            println!("Corbeille déjà vide.");
            return Ok(());
        }
        store.tasks.retain(|t| !t.is_trash);
        save(&store)?;
        println!("Corbeille vidée ({count} tâche{} supprimée{}).", if count > 1 { "s" } else { "" }, if count > 1 { "s" } else { "" });
        return Ok(());
    }

    if trashed.is_empty() {
        println!("Corbeille vide.");
        return Ok(());
    }

    println!("=== Corbeille ===");
    println!("{:<38} {:<30} {:<10} {:<12}", "ID", "TITRE", "PRIORITÉ", "STATUT");
    println!("{}", "-".repeat(94));
    for task in &trashed {
        println!(
            "{:<38} {:<30} {:<10} {:<12}",
            task.id,
            truncate(&task.title, 28),
            task.priority,
            task.status,
        );
    }

    Ok(())
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max - 3])
    } else {
        s.to_string()
    }
}
