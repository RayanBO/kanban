use chrono::Utc;
use uuid::Uuid;

use crate::store::{load, save};
use crate::models::User;

pub fn run(username: &str, pic: Option<&str>) -> Result<(), String> {
    let mut store = load()?;

    let id = Uuid::new_v4().to_string();
    let user = User {
        id: id.clone(),
        username: username.to_string(),
        pic: pic.map(|s| s.to_string()),
        created_at: Utc::now(),
    };

    store.users.push(user);
    save(&store)?;

    println!("{id}");
    Ok(())
}
