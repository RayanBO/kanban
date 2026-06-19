use crate::models::Status;
use crate::store::{load, save};

pub fn run(task_id: &str, new_status: Status) -> Result<(), String> {
    let mut store = load()?;

    let task = store
        .tasks
        .iter_mut()
        .find(|t| t.id == task_id)
        .ok_or_else(|| format!("Task ID inconnu: {task_id}"))?;

    let old_status = task.status.clone();
    task.status = new_status.clone();
    save(&store)?;

    println!("Task {task_id}: {old_status} → {new_status}");
    Ok(())
}
