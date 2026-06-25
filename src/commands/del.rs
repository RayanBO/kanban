use crate::store::{load, save};

pub fn run(task_id: &str) -> Result<(), String> {
    let mut store = load()?;

    let task = store
        .tasks
        .iter_mut()
        .find(|t| t.id == task_id)
        .ok_or_else(|| format!("Task ID inconnu: {task_id}"))?;

    if store.config.use_trash {
        if task.is_trash {
            return Err(format!("Task {task_id} est déjà dans la corbeille."));
        }
        task.is_trash = true;
        save(&store)?;
        println!("Task {task_id} déplacée vers la corbeille.");
    } else {
        store.tasks.retain(|t| t.id != task_id);
        save(&store)?;
        println!("Task {task_id} supprimée définitivement.");
    }

    Ok(())
}
