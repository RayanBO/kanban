use crate::models::{Priority, Status};
use crate::store::load;

pub fn run(priority: Option<Priority>, status: Option<Status>) -> Result<(), String> {
    let store = load()?;

    let tasks: Vec<_> = store
        .tasks
        .iter()
        .filter(|t| !t.is_trash)
        .filter(|t| priority.as_ref().map_or(true, |p| &t.priority == p))
        .filter(|t| status.as_ref().map_or(true, |s| &t.status == s))
        .collect();

    if tasks.is_empty() {
        println!("Aucune tâche trouvée.");
        return Ok(());
    }

    println!(
        "{:<38} {:<30} {:<10} {:<12} {}",
        "ID", "TITRE", "PRIORITÉ", "STATUT", "ASSIGNÉ À"
    );
    println!("{}", "-".repeat(100));

    for task in tasks {
        let assigned = if task.assigned_to.is_empty() {
            "-".to_string()
        } else {
            task.assigned_to.join(", ")
        };
        println!(
            "{:<38} {:<30} {:<10} {:<12} {}",
            task.id,
            truncate(&task.title, 28),
            task.priority,
            task.status,
            assigned
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
