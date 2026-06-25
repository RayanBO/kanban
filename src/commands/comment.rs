use uuid::Uuid;

use crate::models::Comment;
use crate::store;

pub fn add(task_id: &str, content: &str, author_id: Option<&str>) -> Result<(), String> {
    let mut s = store::load()?;

    if !s.tasks.iter().any(|t| t.id == task_id) {
        return Err(format!("Tâche introuvable: {task_id}"));
    }

    let author_id = author_id
        .filter(|v| !v.trim().is_empty())
        .filter(|v| s.users.iter().any(|u| u.id == *v));

    let comment = Comment {
        id: Uuid::new_v4().to_string(),
        task_id: task_id.to_string(),
        author_id: author_id.map(|s| s.to_string()),
        content: content.trim().to_string(),
        created_at: chrono::Utc::now(),
        updated_at: None,
    };

    println!("  Commentaire ajouté: {}", comment.id);
    s.comments.push(comment);
    store::save(&s)
}

pub fn del(id: &str) -> Result<(), String> {
    let mut s = store::load()?;

    let initial_len = s.comments.len();
    s.comments.retain(|c| c.id != id);

    if s.comments.len() == initial_len {
        return Err(format!("Commentaire introuvable: {id}"));
    }

    println!("  Commentaire supprimé: {id}");
    store::save(&s)
}
