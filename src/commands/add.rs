use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::{Priority, Status, Task};
use crate::tags::normalize_tags;
use crate::store::{load, save};

pub fn run(
    title: &str,
    priority: Priority,
    tags: Vec<String>,
    assigned_to: Vec<String>,
    due_date: Option<DateTime<Utc>>,
) -> Result<(), String> {
    let mut store = load()?;

    // Valider que les user IDs existent
    let known_ids: Vec<&str> = store.users.iter().map(|u| u.id.as_str()).collect();
    for uid in &assigned_to {
        if !known_ids.contains(&uid.as_str()) {
            return Err(format!("User ID inconnu: {uid}"));
        }
    }

    let id = Uuid::new_v4().to_string();
    let task = Task {
        id: id.clone(),
        title: title.to_string(),
        priority,
        status: Status::Todo,
        assigned_to,
        tags: normalize_tags(tags),
        created_at: Utc::now(),
        due_date,
        is_trash: false,
    };

    store.tasks.push(task);
    save(&store)?;

    println!("{id}");
    Ok(())
}
