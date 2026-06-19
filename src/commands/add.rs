use chrono::Utc;
use uuid::Uuid;

use crate::models::{Priority, Status, Task};
use crate::store::{load, save};

pub fn run(title: &str, priority: Priority, assigned_to: Vec<String>) -> Result<(), String> {
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
        created_at: Utc::now(),
        is_trash: false,
    };

    store.tasks.push(task);
    save(&store)?;

    println!("{id}");
    Ok(())
}
