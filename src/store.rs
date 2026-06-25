use std::fs;
use std::path::{Path, PathBuf};

use crate::models::{Config, Status, Store};

pub fn kanban_dir() -> PathBuf {
    Path::new(".kanban").to_path_buf()
}

pub fn kanban_path() -> PathBuf {
    kanban_dir().join("kanban.md")
}

pub fn config_path() -> PathBuf {
    kanban_dir().join("kb-config.yaml")
}

pub fn load_config() -> Result<Config, String> {
    let path = config_path();
    if !path.exists() {
        return Ok(Config::default());
    }
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Lecture config échouée: {e}"))?;
    serde_yaml::from_str(&content)
        .map_err(|e| format!("Parse config échoué: {e}"))
}

fn ensure_kanban_dir() -> Result<(), String> {
    let dir = kanban_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .map_err(|e| format!("Création dossier .kanban échouée: {e}"))?;
    }
    Ok(())
}

pub fn save_config(config: &Config) -> Result<(), String> {
    ensure_kanban_dir()?;
    let yaml = serde_yaml::to_string(config)
        .map_err(|e| format!("Sérialisation config échouée: {e}"))?;
    fs::write(config_path(), yaml)
        .map_err(|e| format!("Écriture config échouée: {e}"))
}

pub fn is_initialized() -> bool {
    kanban_path().exists()
}

pub fn load() -> Result<Store, String> {
    if !is_initialized() {
        return Err("kanban.md non trouvé. Lance `kb init` d'abord.".to_string());
    }
    let content = fs::read_to_string(kanban_path())
        .map_err(|e| format!("Lecture kanban.md échouée: {e}"))?;
    let yaml = extract_frontmatter(&content)?;
    serde_yaml::from_str(&yaml)
        .map_err(|e| format!("Parse YAML échoué: {e}"))
}

pub fn save(store: &Store) -> Result<(), String> {
    ensure_kanban_dir()?;
    let yaml = serde_yaml::to_string(store)
        .map_err(|e| format!("Sérialisation YAML échouée: {e}"))?;
    let body = generate_markdown(store);
    let content = format!("---\n{yaml}---\n\n{body}");
    fs::write(kanban_path(), content)
        .map_err(|e| format!("Écriture kanban.md échouée: {e}"))
}

fn extract_frontmatter(content: &str) -> Result<String, String> {
    let normalized = content.strip_prefix('\u{feff}').unwrap_or(content).replace("\r\n", "\n");
    let after_first = normalized
        .strip_prefix("---\n")
        .ok_or("kanban.md: frontmatter YAML manquant (doit commencer par ---)")?;
    let end = after_first
        .find("\n---\n")
        .ok_or("kanban.md: frontmatter non fermé")?;
    Ok(after_first[..end + 1].to_string())
}

fn resolve_users(store: &Store, ids: &[String]) -> String {
    if ids.is_empty() {
        return "-".to_string();
    }
    ids.iter()
        .map(|id| {
            store
                .users
                .iter()
                .find(|u| &u.id == id)
                .map(|u| u.username.as_str())
                .unwrap_or(id.as_str())
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn generate_markdown(store: &Store) -> String {
    let mut out = String::new();

    out.push_str("# Kanban\n\n");

    out.push_str("## Users\n\n");
    out.push_str("| ID | Username | Pic |\n");
    out.push_str("|----|----------|-----|\n");
    for user in &store.users {
        let pic = user.pic.as_deref().unwrap_or("-");
        out.push_str(&format!(
            "| `{}` | {} | {} |\n",
            user.id,
            user.username,
            pic
        ));
    }
    out.push('\n');

    out.push_str("## Tasks\n\n");

    for (status, header) in &[
        (Status::Todo, "### Todo"),
        (Status::InProgress, "### In Progress"),
        (Status::Done, "### Done"),
    ] {
        out.push_str(header);
        out.push_str("\n\n");
        out.push_str("| ID | Title | Priority | Due | Assigned |\n");
        out.push_str("|----|-------|----------|-----|----------|\n");
        for task in store.tasks.iter().filter(|t| &t.status == status && !t.is_trash) {
            let due = task.due_date.map_or("-".to_string(), |d| d.format("%Y-%m-%d").to_string());
            out.push_str(&format!(
                "| `{}` | {} | {} | {} | {} |\n",
                task.id,
                task.title,
                task.priority,
                due,
                resolve_users(store, &task.assigned_to)
            ));
        }
        out.push('\n');
    }

    out
}
