use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl std::str::FromStr for Priority {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            _ => Err(format!("priority invalide: {s}. Utiliser low|medium|high")),
        }
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "low"),
            Priority::Medium => write!(f, "medium"),
            Priority::High => write!(f, "high"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl std::str::FromStr for Status {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "todo" => Ok(Status::Todo),
            "in-progress" | "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(format!("statut invalide: {s}. Utiliser todo|in-progress|done")),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::Todo => write!(f, "todo"),
            Status::InProgress => write!(f, "in-progress"),
            Status::Done => write!(f, "done"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub priority: Priority,
    pub status: Status,
    pub assigned_to: Vec<String>,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub is_trash: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub use_trash: bool,
    pub theme_dashboard: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            use_trash: true,
            theme_dashboard: "dark".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub pic: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Store {
    pub tasks: Vec<Task>,
    pub users: Vec<User>,
}
