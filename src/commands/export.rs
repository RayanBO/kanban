use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;

use crate::models::{Comment, Status, Store, Task};
use crate::store;
use crate::store::load;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Json,
    Markdown,
}

impl ExportFormat {
    pub fn from_flags(json: bool, md: bool) -> Result<Self, String> {
        match (json, md) {
            (true, false) => Ok(Self::Json),
            (false, true) => Ok(Self::Markdown),
            (true, true) => Err("Choisis un seul format: --json ou --md.".to_string()),
            (false, false) => Ok(Self::Json),
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Markdown => "md",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Json => "JSON",
            Self::Markdown => "Markdown",
        }
    }
}

pub fn run(format: ExportFormat, to_file: Option<&str>) -> Result<(), String> {
    let store = load()?;
    let path = write_store_export(&store, format, to_file)?;
    println!("Exporté {} → {}", format.label(), path.display());
    Ok(())
}

pub fn write_store_export(
    store_data: &Store,
    format: ExportFormat,
    to_file: Option<&str>,
) -> Result<PathBuf, String> {
    let path = to_file
        .map(PathBuf::from)
        .unwrap_or_else(|| default_export_path(format));
    let output = render_store_export(store_data, format)?;
    write_export_content(&path, &output)?;
    Ok(path)
}

pub fn render_store_export(store_data: &Store, format: ExportFormat) -> Result<String, String> {
    match format {
        ExportFormat::Json => serde_json::to_string_pretty(store_data)
            .map_err(|e| format!("Sérialisation JSON échouée: {e}")),
        ExportFormat::Markdown => Ok(render_markdown(store_data)),
    }
}

pub fn default_export_path(format: ExportFormat) -> PathBuf {
    store::kanban_dir().join(format!("kb-export.{}", format.extension()))
}

pub fn download_filename(format: ExportFormat) -> String {
    format!("kb-export.{}", format.extension())
}

pub fn content_type(format: ExportFormat) -> &'static str {
    match format {
        ExportFormat::Json => "application/json; charset=utf-8",
        ExportFormat::Markdown => "text/markdown; charset=utf-8",
    }
}

pub fn write_export_content(path: &Path, content: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Création du dossier d'export échouée: {e}"))?;
    }

    fs::write(path, content).map_err(|e| format!("Écriture {} échouée: {e}", path.display()))
}

fn render_markdown(store_data: &Store) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "# kb export");
    let _ = writeln!(out);
    let _ = writeln!(out, "- Version: {}", store_data.version);
    let _ = writeln!(out, "- Exporté le: {}", Utc::now().to_rfc3339());
    let _ = writeln!(out, "- Tasks: {}", store_data.tasks.len());
    let _ = writeln!(out, "- Users: {}", store_data.users.len());
    let _ = writeln!(out, "- Comments: {}", store_data.comments.len());
    let _ = writeln!(out);

    let _ = writeln!(out, "## Config");
    let _ = writeln!(out, "- use_trash: {}", store_data.config.use_trash);
    let _ = writeln!(
        out,
        "- theme_dashboard: {}",
        escape_md_text(&store_data.config.theme_dashboard),
    );
    let _ = writeln!(out);

    let _ = writeln!(out, "## Users");
    if store_data.users.is_empty() {
        let _ = writeln!(out, "_Aucun utilisateur._");
    } else {
        let _ = writeln!(out, "| ID | Username | Pic |");
        let _ = writeln!(out, "|---|---|---|");
        for user in &store_data.users {
            let pic = user.pic.as_deref().filter(|v| !v.trim().is_empty()).unwrap_or("-");
            let _ = writeln!(
                out,
                "| `{}` | {} | {} |",
                user.id,
                escape_md_cell(&user.username),
                escape_md_cell(pic),
            );
        }
    }
    let _ = writeln!(out);

    let active_tasks: Vec<&Task> = store_data.tasks.iter().filter(|t| !t.is_trash).collect();
    let trash_tasks: Vec<&Task> = store_data.tasks.iter().filter(|t| t.is_trash).collect();

    render_task_status_sections(&mut out, store_data, &active_tasks);
    if !trash_tasks.is_empty() {
        let _ = writeln!(out, "## Trash");
        render_task_table(&mut out, store_data, &trash_tasks);
    }

    render_comments_section(&mut out, store_data);
    out
}

fn render_task_status_sections(out: &mut String, store_data: &Store, tasks: &[&Task]) {
    for (status, title) in [
        (Status::Todo, "Todo"),
        (Status::InProgress, "In progress"),
        (Status::Done, "Done"),
    ] {
        let grouped: Vec<&Task> = tasks
            .iter()
            .copied()
            .filter(|task| task.status == status)
            .collect();
        if grouped.is_empty() {
            continue;
        }
        let _ = writeln!(out, "## {}", title);
        render_task_table(out, store_data, &grouped);
    }
}

fn render_task_table(out: &mut String, store_data: &Store, tasks: &[&Task]) {
    let _ = writeln!(out, "| ID | Title | Priority | Due | Assigned | Tags | Comments |");
    let _ = writeln!(out, "|---|---|---|---|---|---|---|");
    for task in tasks {
        let due = task
            .due_date
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "-".to_string());
        let assigned = render_assigned_users(store_data, task);
        let tags = if task.tags.is_empty() {
            "-".to_string()
        } else {
            task.tags
                .iter()
                .map(|tag| escape_md_cell(tag))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let comments = store_data
            .comments
            .iter()
            .filter(|comment| comment.task_id == task.id)
            .count();
        let _ = writeln!(
            out,
            "| `{}` | {} | {} | {} | {} | {} | {} |",
            task.id,
            escape_md_cell(&task.title),
            task.priority,
            due,
            assigned,
            tags,
            comments,
        );
    }
    let _ = writeln!(out);
}

fn render_assigned_users(store_data: &Store, task: &Task) -> String {
    if task.assigned_to.is_empty() {
        return "-".to_string();
    }

    let names = task
        .assigned_to
        .iter()
        .map(|user_id| {
            store_data
                .users
                .iter()
                .find(|user| &user.id == user_id)
                .map(|user| user.username.as_str())
                .unwrap_or(user_id.as_str())
        })
        .map(escape_md_cell)
        .collect::<Vec<_>>();
    names.join(", ")
}

fn render_comments_section(out: &mut String, store_data: &Store) {
    if store_data.comments.is_empty() {
        return;
    }

    let _ = writeln!(out, "## Comments");
    for task in &store_data.tasks {
        let comments: Vec<&Comment> = store_data
            .comments
            .iter()
            .filter(|comment| comment.task_id == task.id)
            .collect();
        if comments.is_empty() {
            continue;
        }
        let _ = writeln!(out, "### {}", escape_md_text(&task.title));
        let _ = writeln!(out, "_Task ID: `{}`_", task.id);
        for comment in comments {
            let author = comment
                .author_id
                .as_deref()
                .and_then(|author_id| {
                    store_data
                        .users
                        .iter()
                        .find(|user| user.id == author_id)
                        .map(|user| user.username.as_str())
                })
                .unwrap_or("Anonyme");
            let timestamp = comment.created_at.format("%Y-%m-%d %H:%M UTC");
            let _ = writeln!(out, "- {} — {}: {}", timestamp, author, escape_md_text(&comment.content));
        }
        let _ = writeln!(out);
    }
}

fn escape_md_cell(value: &str) -> String {
    escape_md_text(value).replace('|', "\\|")
}

fn escape_md_text(value: &str) -> String {
    value.replace('\r', " ").replace('\n', " ").trim().to_string()
}
