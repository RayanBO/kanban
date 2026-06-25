use std::io::{self, Write};

use crate::models::{Config, Store};
use crate::store::{data_path, is_initialized, save};

pub fn run(use_trash: Option<bool>, _no_init_dashboard: bool) -> Result<(), String> {
    if is_initialized() {
        return Err("kb-data.yaml existe déjà dans ce dossier.".to_string());
    }

    let interactive = use_trash.is_none();

    if interactive {
        print!("Initialiser kanban avec config par défaut ? (Y/n) ");
        io::stdout().flush().map_err(|e| e.to_string())?;
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        let answer = input.trim().to_lowercase();
        if !answer.is_empty() && answer != "y" {
            println!("Initialisation annulée.");
            return Ok(());
        }
    }

    let mut config = Config::default();
    if let Some(v) = use_trash {
        config.use_trash = v;
    }

    let mut store = Store::default();
    store.config = config;
    save(&store)?;

    println!("Kanban initialisé → {}", data_path().display());
    Ok(())
}
