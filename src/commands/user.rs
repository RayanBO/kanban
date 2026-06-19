use chrono::Utc;
use uuid::Uuid;

use crate::models::User;
use crate::store::{load, save};

pub fn add(username: &str, pic: Option<&str>) -> Result<(), String> {
    let mut store = load()?;

    let id = Uuid::new_v4().to_string();
    store.users.push(User {
        id: id.clone(),
        username: username.to_string(),
        pic: pic.map(|s| s.to_string()),
        created_at: Utc::now(),
    });
    save(&store)?;

    println!("{id}");
    Ok(())
}

pub fn put(id: &str, username: Option<&str>, pic: Option<&str>) -> Result<(), String> {
    let mut store = load()?;

    let user = store
        .users
        .iter_mut()
        .find(|u| u.id == id)
        .ok_or_else(|| format!("User ID inconnu: {id}"))?;

    if username.is_none() && pic.is_none() {
        return Err("Spécifie au moins --username ou --pic.".to_string());
    }
    if let Some(name) = username {
        user.username = name.to_string();
    }
    if let Some(path) = pic {
        user.pic = Some(path.to_string());
    }
    save(&store)?;

    println!("User {id} mis à jour.");
    Ok(())
}

pub fn del(id: &str) -> Result<(), String> {
    let mut store = load()?;

    let exists = store.users.iter().any(|u| u.id == id);
    if !exists {
        return Err(format!("User ID inconnu: {id}"));
    }

    store.users.retain(|u| u.id != id);
    for task in store.tasks.iter_mut() {
        task.assigned_to.retain(|uid| uid != id);
    }
    save(&store)?;

    println!("User {id} supprimé.");
    Ok(())
}

pub fn show() -> Result<(), String> {
    let store = load()?;

    if store.users.is_empty() {
        println!("Aucun utilisateur.");
        return Ok(());
    }

    println!("{:<38} {:<20} {}", "ID", "USERNAME", "PIC");
    println!("{}", "-".repeat(70));
    for user in &store.users {
        let pic = user.pic.as_deref().unwrap_or("-");
        println!("{:<38} {:<20} {}", user.id, user.username, pic);
    }
    Ok(())
}
