use std::env;

use crate::server;
use crate::store::is_initialized;

fn emoji_w(c: char) -> usize {
    let u = c as u32;
    if u >= 0x1F000 && u <= 0x1FFFF {
        2
    } else if u == 0x2699 || u == 0x2705 || u == 0x2795 || u == 0x2796 {
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

fn banner(url: &str, watch: bool) {
    let raw = env::current_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();
    let folder = raw.rsplit('\\').next().unwrap_or("?");

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

    println!();
    println!("  ┌{}┐", "─".repeat(w));
    sp();
    sp();
    for l in &logo {
        println!("{}", line(l));
    }
    sp();
    sp();
    println!("{}", line("    ◆  Dashboard lancé  ✅"));
    println!("{}", line(&format!("    v{}", env!("CARGO_PKG_VERSION"))));
    sp();
    println!("{}", line(&format!("    📁  {}", folder)));
    println!("{}", line(&format!("    🔗  {}", url)));
    println!("{}", line("    ⚙   Serveur Rust"));
    if watch {
        println!("{}", line("    👀  Mode watch actif"));
    }
    sp();
    println!("{}", line("    Appuie sur Entrée pour quitter"));
    sp();
    println!("  └{}┘", "─".repeat(w));
    println!();
}

pub fn run(watch: bool) -> Result<(), String> {
    if !is_initialized() {
        return Err("Aucun projet Kanban ici. Exécute 'kb init' d'abord.".to_string());
    }

    let port = server::find_port(5522);
    let url = format!("http://localhost:{}", port);

    banner(&url, watch);

    let rt = tokio::runtime::Runtime::new().map_err(|e| format!("Échec du runtime: {e}"))?;

    rt.block_on(server::run_server(port, watch))
}
