use clap::{CommandFactory, Parser, Subcommand};

mod commands;
mod embed;
mod models;
mod tags;
mod server;
mod store;

use models::{Priority, Status};

#[cfg(windows)]
fn set_console_utf8() {
    use windows_sys::Win32::System::Console::{SetConsoleCP, SetConsoleOutputCP};

    unsafe {
        SetConsoleCP(65001);
        SetConsoleOutputCP(65001);
    }
}

#[cfg(not(windows))]
fn set_console_utf8() {}

fn parse_date(s: &str) -> Result<chrono::DateTime<chrono::Utc>, String> {
    let d = chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| format!("Date invalide: {s}. Utilise YYYY-MM-DD."))?;
    Ok(d.and_hms_opt(0, 0, 0).unwrap().and_utc())
}

#[derive(Parser)]
#[command(name = "kb", about = "Kanban CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialiser kb-data.yaml dans le dossier courant
    Init {
        #[arg(long, default_missing_value = "true", num_args = 0..=1)]
        use_trash: Option<bool>,
        #[arg(long)]
        no_init_dashboard: bool,
    },

    /// Ajouter une tâche
    Add {
        title: String,
        #[arg(short = 'p', long, default_value = "medium")]
        priority: String,
        #[arg(long = "tag", value_delimiter = ',')]
        tags: Vec<String>,
        #[arg(long = "to", value_delimiter = ',')]
        assigned_to: Vec<String>,
        #[arg(long)]
        due: Option<String>,
    },

    /// Assigner des utilisateurs à une tâche existante
    Assign {
        task_id: String,
        #[arg(long = "to", value_delimiter = ',')]
        assigned_to: Vec<String>,
    },

    /// Gérer les utilisateurs
    User {
        #[command(subcommand)]
        action: UserAction,
    },

    /// KPIs globaux
    Status,

    /// Lister les tâches
    List {
        #[arg(short = 'p', long)]
        priority: Option<String>,
        #[arg(short = 's', long)]
        status: Option<String>,
        #[arg(long = "tag", value_delimiter = ',')]
        tags: Vec<String>,
    },

    /// Modifier le titre, la priorité ou la date d'échéance d'une tâche
    Edit {
        id: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(short = 'p', long)]
        priority: Option<String>,
        #[arg(long = "tag", value_delimiter = ',')]
        tags: Vec<String>,
        #[arg(long)]
        clear_tags: bool,
        #[arg(long)]
        due: Option<String>,
        #[arg(long)]
        clear_due: bool,
    },

    /// Changer le statut d'une tâche
    Move {
        task_id: String,
        new_status: String,
    },

    /// Exporter toutes les données
    #[command(alias = "data")]
    Export {
        #[arg(long, conflicts_with = "md")]
        json: bool,
        #[arg(long, conflicts_with = "json")]
        md: bool,
        #[arg(long = "to-file")]
        to_file: Option<String>,
    },

    /// Lister les tags existants
    Tags,

    /// Supprimer une tâche (corbeille si activée)
    Del {
        task_id: String,
    },

    /// Gérer les commentaires
    Comment {
        #[command(subcommand)]
        action: CommentAction,
    },

    /// Gérer la configuration
    Config {
        #[arg(long = "set", value_name = "KEY=VALUE")]
        set: Vec<String>,
    },

    /// Gérer la corbeille
    Trash {
        #[arg(long)]
        restore: Option<String>,
        #[arg(long)]
        clean_all: bool,
    },

    /// Installer kb dans le PATH utilisateur
    Install,

    /// Lancer le dashboard web
    Dashboard {
        #[arg(long)]
        watch: bool,
    },
}

#[derive(Subcommand)]
enum CommentAction {
    /// Ajouter un commentaire à une tâche
    Add {
        task_id: String,
        content: String,
        #[arg(long)]
        author_id: Option<String>,
    },
    /// Supprimer un commentaire par son ID
    Del {
        id: String,
    },
}

#[derive(Subcommand)]
enum UserAction {
    /// Créer un utilisateur
    Add {
        username: String,
        #[arg(long)]
        pic: Option<String>,
    },
    /// Modifier un utilisateur
    Put {
        id: String,
        #[arg(long)]
        username: Option<String>,
        #[arg(long)]
        pic: Option<String>,
    },
    /// Supprimer un utilisateur
    Del {
        id: String,
    },
    /// Afficher les utilisateurs
    Show,
}

fn main() {
    set_console_utf8();

    if std::env::args().len() <= 1 {
        if commands::install::is_installed() {
            let mut cmd = Cli::command();
            let _ = cmd.print_help();
            println!();
            return;
        }
        let result = commands::install::run();
        if let Err(e) = result {
            eprintln!("Erreur: {e}");
        }
        println!("  Appuie sur une touche pour quitter...");
        let _ = std::io::stdin().read_line(&mut String::new());
        return;
    }

    let cli = Cli::parse();

    let result = match cli.command {
        Command::Init { use_trash, no_init_dashboard } => {
            commands::init::run(use_trash, no_init_dashboard)
        }

        Command::Add { title, priority, tags, assigned_to, due } => {
            let p = priority.parse::<Priority>().unwrap_or_else(|e| {
                eprintln!("Erreur: {e}");
                std::process::exit(1);
            });
            let tags = tags::normalize_tags(tags);
            let assigned_to: Vec<String> = assigned_to.into_iter().filter(|s| !s.is_empty()).collect();
            let due_date = due.as_deref().map(parse_date).transpose().unwrap_or_else(|e| {
                eprintln!("Erreur: {e}");
                std::process::exit(1);
            });
            commands::add::run(&title, p, tags, assigned_to, due_date)
        }

        Command::Assign { task_id, assigned_to } => {
            let assigned_to: Vec<String> = assigned_to.into_iter().filter(|s| !s.is_empty()).collect();
            commands::assign::run(&task_id, assigned_to)
        }

        Command::User { action } => match action {
            UserAction::Add { username, pic } => {
                commands::user::add(&username, pic.as_deref())
            }
            UserAction::Put { id, username, pic } => {
                commands::user::put(&id, username.as_deref(), pic.as_deref())
            }
            UserAction::Del { id } => commands::user::del(&id),
            UserAction::Show => commands::user::show(),
        },

        Command::Status => commands::status::run(),

        Command::List { priority, status, tags } => {
            let p = priority.as_deref().map(|s| {
                s.parse::<Priority>().unwrap_or_else(|e| {
                    eprintln!("Erreur: {e}");
                    std::process::exit(1);
                })
            });
            let st = status.as_deref().map(|s| {
                s.parse::<Status>().unwrap_or_else(|e| {
                    eprintln!("Erreur: {e}");
                    std::process::exit(1);
                })
            });
            let tags = tags::normalize_tags(tags);
            commands::list::run(p, st, tags)
        }

        Command::Edit { id, title, priority, tags, clear_tags, due, clear_due } => {
            let p = priority.as_deref().map(|s| {
                s.parse::<Priority>().unwrap_or_else(|e| {
                    eprintln!("Erreur: {e}");
                    std::process::exit(1);
                })
            });
            let tags = if clear_tags {
                Some(Vec::new())
            } else {
                let tags = tags::normalize_tags(tags);
                if tags.is_empty() { None } else { Some(tags) }
            };
            let due_date = if clear_due {
                Some(None)
            } else {
                due.as_deref().map(|s| parse_date(s).map(Some)).transpose().unwrap_or_else(|e| {
                    eprintln!("Erreur: {e}");
                    std::process::exit(1);
                })
            };
            commands::edit::run(&id, title.as_deref(), p, tags, due_date)
        }

        Command::Move { task_id, new_status } => {
            let s = new_status.parse::<Status>().unwrap_or_else(|e| {
                eprintln!("Erreur: {e}");
                std::process::exit(1);
            });
            commands::move_task::run(&task_id, s)
        }

        Command::Export { json, md, to_file } => {
            let format = commands::export::ExportFormat::from_flags(json, md).unwrap_or_else(|e| {
                eprintln!("Erreur: {e}");
                std::process::exit(1);
            });
            commands::export::run(format, to_file.as_deref())
        }

        Command::Tags => commands::tags::run(),

        Command::Del { task_id } => commands::del::run(&task_id),

        Command::Comment { action } => match action {
            CommentAction::Add { task_id, content, author_id } => {
                commands::comment::add(&task_id, &content, author_id.as_deref())
            }
            CommentAction::Del { id } => {
                commands::comment::del(&id)
            }
        },

        Command::Config { set } => {
            let pairs: Vec<(String, String)> = set
                .into_iter()
                .map(|s| {
                    let mut parts = s.splitn(2, '=');
                    let key = parts.next().unwrap_or("").to_string();
                    let value = parts.next().unwrap_or("").to_string();
                    (key, value)
                })
                .collect();
            commands::config::run(pairs)
        }

        Command::Trash { restore, clean_all } => {
            commands::trash::run(restore, clean_all)
        }

        Command::Install => commands::install::run(),

        Command::Dashboard { watch } => commands::dashboard::run(watch),
    };

    if let Err(e) = result {
        eprintln!("Erreur: {e}");
        std::process::exit(1);
    }
}
