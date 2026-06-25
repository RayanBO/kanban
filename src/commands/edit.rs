use chrono::{DateTime, Utc};

use crate::models::Priority;
use crate::store::{load, save};

pub fn run(id: &str, title: Option<&str>, priority: Option<Priority>, due_date: Option<Option<DateTime<Utc>>>) -> Result<(), String> {
    if title.is_none() && priority.is_none() && due_date.is_none() {
        return Err("Spécifie au moins --title, --priority ou --due.".to_string());
    }
    let mut store = load()?;
    let task = store
        .tasks
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| format!("Task ID inconnu: {id}"))?;

    if let Some(t) = title {
        task.title = t.to_string();
    }
    if let Some(p) = priority {
        task.priority = p;
    }
    if let Some(d) = due_date {
        task.due_date = d;
    }
    save(&store)?;
    println!("Task {id} mise à jour.");
    Ok(())
}
