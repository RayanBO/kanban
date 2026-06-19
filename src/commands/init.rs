use std::io::{self, Write};

use crate::models::{Config, Store};
use crate::store::{is_initialized, kanban_dir, kanban_path, save, save_config};

pub fn run(use_trash: Option<bool>, no_init_dashboard: bool) -> Result<(), String> {
    if is_initialized() {
        return Err("kanban.md existe déjà dans ce dossier.".to_string());
    }

    let interactive = use_trash.is_none() && !no_init_dashboard;

    if interactive {
        print!("Initialiser kanban avec config par défaut ? (y/n) ");
        io::stdout().flush().map_err(|e| e.to_string())?;
        let mut input = String::new();
        io::stdin().read_line(&mut input).map_err(|e| e.to_string())?;
        if input.trim().to_lowercase() != "y" {
            println!("Initialisation annulée.");
            return Ok(());
        }
    }

    let mut config = Config::default();
    if let Some(v) = use_trash {
        config.use_trash = v;
    }

    let store = Store::default();
    save(&store)?;
    save_config(&config)?;

    if !no_init_dashboard {
        create_dashboard_scripts()?;
    }

    println!("Kanban initialisé → {}", kanban_path().display());
    Ok(())
}

fn create_dashboard_scripts() -> Result<(), String> {
    let dir = kanban_dir();

    let bat = dir.join("dashboard.bat");
    let bat_content = "@echo off\r\ncd /d \"%~dp0..\"\r\nkb status\r\necho.\r\npause\r\n";
    std::fs::write(&bat, bat_content)
        .map_err(|e| format!("Écriture {} échouée: {e}", bat.display()))?;

    let sh = dir.join("dashboard.sh");
    let sh_content = "#!/bin/sh\ncd \"$(dirname \"$0\")/..\"\nkb status\n";
    std::fs::write(&sh, sh_content)
        .map_err(|e| format!("Écriture {} échouée: {e}", sh.display()))?;

    Ok(())
}
