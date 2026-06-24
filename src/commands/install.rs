use std::env;
use std::path::{Path, PathBuf};

pub fn install_dir() -> PathBuf {
    PathBuf::from(env::var("LOCALAPPDATA").unwrap_or_default())
        .join("Programs")
        .join("kb")
}

pub fn is_installed() -> bool {
    let current = env::current_exe().ok();
    let installed = Some(install_dir().join("kb.exe"));
    current == installed
}

fn emoji_w(c: char) -> usize {
    let u = c as u32;
    if u >= 0x1F000 && u <= 0x1FFFF {
        2
    } else if u == 0x2705 || u == 0x2795 || u == 0x2796 {
        2
    } else {
        1
    }
}

fn vis_width(s: &str) -> usize {
    s.chars().filter(|&c| c != '\r').map(emoji_w).sum()
}

fn pad_to(s: &str, w: usize) -> String {
    let clean: String = s.chars().filter(|&c| c != '\r').collect();
    let vw = vis_width(&clean);
    if vw >= w {
        let mut out = String::new();
        let mut v = 0;
        for c in clean.chars() {
            let cw = emoji_w(c);
            if v + cw > w {
                break;
            }
            out.push(c);
            v += cw;
        }
        while v < w {
            out.push(' ');
            v += 1;
        }
        out
    } else {
        let need = w - vw;
        let mut out = String::with_capacity(clean.len() + need);
        out.push_str(&clean);
        for _ in 0..need {
            out.push(' ');
        }
        out
    }
}

#[cfg(windows)]
pub fn run() -> Result<(), String> {
    let dir = install_dir();
    let exe = env::current_exe().map_err(|e| format!("Impossible de localiser le binaire: {e}"))?;

    if !dir.exists() {
        std::fs::create_dir_all(&dir).map_err(|e| format!("Création dossier échouée: {e}"))?;
    }

    let dest = dir.join("kb.exe");
    std::fs::copy(&exe, &dest).map_err(|e| format!("Copie échouée: {e}"))?;

    let user_path = env::var("Path").unwrap_or_default();
    let dir_str = dir.to_string_lossy().to_string();
    let path_updated = !user_path.split(';').any(|p| p == dir_str);

    if path_updated {
        let new_path = format!("{};{}", user_path, dir_str);
        let ps_cmd = format!(
            "[Environment]::SetEnvironmentVariable('Path','{}','User')",
            new_path.replace('\'', "''")
        );
        let output = std::process::Command::new("powershell")
            .args(["-NoProfile", "-Command", &ps_cmd])
            .output()
            .map_err(|e| format!("Échec mise à jour PATH: {e}"))?;
        if !output.status.success() {
            return Err("Échec mise à jour du PATH utilisateur.".to_string());
        }
    }

    banner(&dest, path_updated);

    Ok(())
}

fn banner(path: &Path, path_updated: bool) {
    let raw = path.to_string_lossy();
    let path_short = if let Some(pos) = raw.rfind("\\AppData\\") {
        let tail = &raw[pos + "\\AppData\\".len()..];
        if tail.len() > 25 {
            format!("...{}", &tail[tail.len() - 22..])
        } else {
            format!("...{}", tail)
        }
    } else {
        let max = 28;
        if raw.len() > max {
            format!("...{}", &raw[raw.len() - (max - 3)..])
        } else {
            raw.to_string()
        }
    };

    let w = 48;
    let line = |s: &str| format!("  │{}│", pad_to(s, w));
    let sp = || println!("{}", line(""));

    let logo = [
        "       █   █ ████ █   █ ████  ████ █   █",
        "       █  █  █  █ ██  █ █   █ █  █ ██  █",
        "       ███   ████ █ █ █ ████  ████ █ █ █",
        "       █  █  █  █ █  ██ █   █ █  █ █  ██",
        "       █   █ █  █ █   █ ████  █  █ █   █",
    ];

    let check = if path_updated { "➕" } else { "✓" };
    let path_label = if path_updated {
        "PATH : Ajouté au PATH"
    } else {
        "PATH : Déjà configuré"
    };

    println!();
    println!("  ┌{}┐", "─".repeat(w));
    sp();
    sp();
    for l in &logo {
        println!("{}", line(l));
    }
    sp();
    sp();
    println!("{}", line("    ◆  Installation de Kanban terminée  ✅"));
    println!("{}", line(&format!("    v{}", env!("CARGO_PKG_VERSION"))));
    sp();
    println!("{}", line(&format!("    FICHIER  {}", path_short)));
    println!("{}", line(&format!("    {}  {}", check, path_label)));
    sp();
    println!("{}", line("              ─────────────────────"));
    sp();
    println!("{}", line("    💻  Ouvre un nouveau terminal et tape :"));
    println!("{}", line("              $ kb --version"));
    sp();
    println!("  └{}┘", "─".repeat(w));
    println!();
}

#[cfg(not(windows))]
pub fn run() -> Result<(), String> {
    Err("Installation auto disponible uniquement sur Windows.".to_string())
}
