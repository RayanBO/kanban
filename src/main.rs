use clap::{CommandFactory, Parser, Subcommand};

mod commands;
mod models;
mod server;
mod store;

use models::{Priority, Status};

#[derive(Parser)]
#[command(name = "kb", about = "Kanban CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Initialiser kanban.md dans le dossier courant
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
    },

    /// Changer le statut d'une tâche
    Move {
        task_id: String,
        new_status: String,
    },

    /// Afficher toutes les données en JSON
    Data {
        #[arg(long = "to-file")]
        to_file: Option<String>,
    },

    /// Supprimer une tâche (corbeille si activée)
    Del {
        task_id: String,
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
    Dashboard,
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

        Command::Add { title, priority, assigned_to } => {
            let p = priority.parse::<Priority>().unwrap_or_else(|e| {
                eprintln!("Erreur: {e}");
                std::process::exit(1);
            });
            let assigned_to: Vec<String> = assigned_to.into_iter().filter(|s| !s.is_empty()).collect();
            commands::add::run(&title, p, assigned_to)
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

        Command::List { priority, status } => {
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
            commands::list::run(p, st)
        }

        Command::Move { task_id, new_status } => {
            let s = new_status.parse::<Status>().unwrap_or_else(|e| {
                eprintln!("Erreur: {e}");
                std::process::exit(1);
            });
            commands::move_task::run(&task_id, s)
        }

        Command::Data { to_file } => commands::data::run(to_file.as_deref()),

        Command::Del { task_id } => commands::del::run(&task_id),

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

        Command::Dashboard => commands::dashboard::run(),
    };

    if let Err(e) = result {
        eprintln!("Erreur: {e}");
        std::process::exit(1);
    }
}
