use std::io::{self, Write};
use std::path::PathBuf;

use crate::models::{Config, Store};
use crate::store::{is_initialized, kanban_dir, kanban_path, save, save_config};
use crate::commands::install::install_dir;

fn copy_dir_recursive(src: &PathBuf, dst: &PathBuf) -> Result<(), String> {
    if !dst.exists() {
        std::fs::create_dir_all(dst).map_err(|e| format!("Création dossier {dst:?}: {e}"))?;
    }
    for entry in std::fs::read_dir(src).map_err(|e| format!("Lecture {src:?}: {e}"))? {
        let entry = entry.map_err(|e| format!("Entrée: {e}"))?;
        let path = entry.path();
        let name = entry.file_name();
        let dest = dst.join(&name);
        if path.is_dir() {
            copy_dir_recursive(&path, &dest)?;
        } else {
            std::fs::copy(&path, &dest).map_err(|e| format!("Copie {name:?}: {e}"))?;
        }
    }
    Ok(())
}

fn find_dashboard_source() -> Option<PathBuf> {
    let candidates = [
        PathBuf::from("dashboard").join("out"),
        PathBuf::from(".kanban").join("dashboard"),
        install_dir().join("dashboard"),
    ];
    for c in &candidates {
        if c.exists() && c.is_dir() && c.join("index.html").exists() {
            return Some(c.clone());
        }
    }
    None
}

pub fn run(use_trash: Option<bool>, no_init_dashboard: bool) -> Result<(), String> {
    if is_initialized() {
        return Err("kanban.md existe déjà dans ce dossier.".to_string());
    }

    let interactive = use_trash.is_none() && !no_init_dashboard;

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

    let store = Store::default();
    save(&store)?;
    save_config(&config)?;

    if !no_init_dashboard {
        let dash_dst = kanban_dir().join("dashboard");
        if let Some(src) = find_dashboard_source() {
            copy_dir_recursive(&src, &dash_dst)?;
            println!("  Dashboard copié → {}", dash_dst.display());
        } else {
            println!("  Avertissement: dashboard introuvable. Exécute 'kb install' ou construis 'dashboard/out/' d'abord.");
        }
        create_dashboard_scripts()?;
    }

    println!("Kanban initialisé → {}", kanban_path().display());
    Ok(())
}

fn create_dashboard_scripts() -> Result<(), String> {
    let dir = kanban_dir();

    let bat = dir.join("dashboard.bat");
    let bat_content = "@echo off\r\ncd /d \"%~dp0..\"\r\nkb dashboard\r\n";
    std::fs::write(&bat, bat_content)
        .map_err(|e| format!("Écriture {} échouée: {e}", bat.display()))?;

    let sh = dir.join("dashboard.sh");
    let sh_content = "#!/bin/sh\ncd \"$(dirname \"$0\")/..\"\nkb dashboard\n";
    std::fs::write(&sh, sh_content)
        .map_err(|e| format!("Écriture {} échouée: {e}", sh.display()))?;

    Ok(())
}
