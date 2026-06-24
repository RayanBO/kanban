use crate::store::{load, save};

pub fn set_assignment(task_id: &str, assigned_to: Vec<String>) -> Result<(), String> {
    let mut store = load()?;

    let known_ids: Vec<&str> = store.users.iter().map(|u| u.id.as_str()).collect();
    for uid in &assigned_to {
        if !known_ids.contains(&uid.as_str()) {
            return Err(format!("User ID inconnu: {uid}"));
        }
    }

    let task = store
        .tasks
        .iter_mut()
        .find(|t| t.id == task_id)
        .ok_or_else(|| format!("Task ID inconnu: {task_id}"))?;

    task.assigned_to = assigned_to;
    save(&store)?;

    Ok(())
}

pub fn run(task_id: &str, assigned_to: Vec<String>) -> Result<(), String> {
    set_assignment(task_id, assigned_to)?;
    println!("Task {task_id} assignée.");
    Ok(())
}
