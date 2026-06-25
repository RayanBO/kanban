use crate::models::{Priority, Status};
use crate::store::load;
use crate::tags::{format_tags, tags_match};

pub fn run(priority: Option<Priority>, status: Option<Status>, tags: Vec<String>) -> Result<(), String> {
    let store = load()?;

    let tasks: Vec<_> = store
        .tasks
        .iter()
        .filter(|t| !t.is_trash)
        .filter(|t| priority.as_ref().map_or(true, |p| &t.priority == p))
        .filter(|t| status.as_ref().map_or(true, |s| &t.status == s))
        .filter(|t| tags_match(&t.tags, &tags))
        .collect();

    if tasks.is_empty() {
        println!("Aucune tâche trouvée.");
        return Ok(());
    }

    println!(
        "{:<38} {:<30} {:<10} {:<12} {:<24} {}",
        "ID", "TITRE", "PRIORITÉ", "STATUT", "TAGS", "ASSIGNÉ À"
    );
    println!("{}", "-".repeat(126));

    for task in tasks {
        let assigned = if task.assigned_to.is_empty() {
            "-".to_string()
        } else {
            task.assigned_to
                .iter()
                .map(|id| {
                    store
                        .users
                        .iter()
                        .find(|u| &u.id == id)
                        .map(|u| u.username.as_str())
                        .unwrap_or(id.as_str())
                })
                .collect::<Vec<_>>()
                .join(", ")
        };
        println!(
            "{:<38} {:<30} {:<10} {:<12} {:<24} {}",
            task.id,
            truncate(&task.title, 28),
            task.priority,
            task.status,
            truncate(&format_tags(&task.tags), 22),
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
