use crate::models::{Priority, Status};
use crate::store::load;

pub fn run() -> Result<(), String> {
    let store = load()?;

    let total = store.tasks.iter().filter(|t| !t.is_trash).count();
    let todo = store.tasks.iter().filter(|t| t.status == Status::Todo && !t.is_trash).count();
    let in_progress = store.tasks.iter().filter(|t| t.status == Status::InProgress && !t.is_trash).count();
    let done = store.tasks.iter().filter(|t| t.status == Status::Done && !t.is_trash).count();
    let trashed = store.tasks.iter().filter(|t| t.is_trash).count();

    let high = store.tasks.iter().filter(|t| t.priority == Priority::High && !t.is_trash).count();
    let medium = store.tasks.iter().filter(|t| t.priority == Priority::Medium && !t.is_trash).count();
    let low = store.tasks.iter().filter(|t| t.priority == Priority::Low && !t.is_trash).count();

    let users = store.users.len();

    let done_pct = if total > 0 {
        (done * 100) / total
    } else {
        0
    };

    println!("=== Kanban Status ===");
    println!();
    println!("Tâches       : {total}");
    println!("  todo       : {todo}");
    println!("  in-progress: {in_progress}");
    println!("  done       : {done} ({done_pct}%)");
    if trashed > 0 {
        println!("  corbeille  : {trashed}");
    }
    println!();
    println!("Priorités");
    println!("  high       : {high}");
    println!("  medium     : {medium}");
    println!("  low        : {low}");
    println!();
    println!("Utilisateurs : {users}");

    Ok(())
}
