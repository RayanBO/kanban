# Kanban CLI + Dashboard — Project Context

## Overview
Kanban task manager: Rust CLI (`kb`) + HTML/CSS/JS web dashboard. Stockage dans `.kanban/kanban.md` (YAML frontmatter + Markdown tables). Dashboard est **embarqué dans le binaire** via `rust-embed` — zéro fichier à copier.

## Architecture
```
kb (Rust CLI) ←→ .kanban/kanban.md (YAML+Markdown)
                  ↕ Rust HTTP server (axum, intégré dans kb CLI)
                    ↕ Dashboard HTML/CSS/JS embarqué (rust-embed)
```
Dashboard est un simple fichier HTML + CSS + JS vanilla, **embarqué dans le binaire** à la compilation. `kb dashboard` lance un serveur Rust (axum) qui sert les fichiers embarqués + gère l'API directement. Aucune dépendance Node.js/npm.

## Project Structure
```
kanban/
├── src/
│   ├── main.rs                  # CLI entry, clap commands
│   ├── embed.rs                 # rust-embed: DashboardAssets struct (embarque dashboard/)
│   ├── models.rs                # Task, User, Store, Config, Status, Priority
│   ├── store.rs                 # load/save kanban.md, config
│   ├── server.rs                # Rust HTTP server (axum): API + embedded files
│   └── commands/
│       ├── mod.rs
│       ├── add.rs               # kb add <title> -p <prio> --to <users>
│       ├── config.rs            # kb config --set key=val
│       ├── dashboard.rs         # kb dashboard (tokio runtime + axum server)
│       ├── data.rs              # kb data (JSON dump)
│       ├── del.rs               # kb del <id>
│       ├── init.rs              # kb init (interactive + flags)
│       ├── install.rs           # kb install (Windows PATH)
│       ├── list.rs              # kb list [-p] [-s]
│       ├── move_task.rs         # kb move <id> <status>
│       ├── status.rs            # kb status (KPIs)
│       ├── trash.rs             # kb trash [--restore] [--clean-all]
│       └── user.rs              # kb user add/put/del/show
├── dashboard/                   # HTML/CSS/JS vanilla — embarqué dans le binaire
│   ├── index.html               # Structure HTML + JS inline (drag & drop, modals, state)
│   ├── style.css                # Tous les styles (CSS variables, themes, responsive)
│   └── fonts/                   # Polices auto-hébergées (woff2)
├── Cargo.toml                   # version 1.1.0
└── README.md
```

## Data Model
```rust
struct Task {
    id: String,              // UUID v4
    title: String,
    priority: Priority,       // low | medium | high
    status: Status,           // todo | in-progress | done
    assigned_to: Vec<String>, // user IDs
    created_at: DateTime<Utc>,
    is_trash: bool,           // #[serde(default)] for backward compat
}
struct User { id, username, pic, created_at }
struct Config { use_trash: bool, theme_dashboard: String }
struct Store { tasks: Vec<Task>, users: Vec<User> }
```

## Storage
- `.kanban/kanban.md` — YAML frontmatter (serde) + Markdown tables
- `.kanban/kb-config.yaml` — configuration
- `kanban_dir()` = `Path::new(".kanban")`
- `save()` creates `.kanban/` if missing
- `is_initialized()` checks `.kanban/kanban.md` exists

## CLI Commands
| Command | Description |
|---|---|
| `kb install` | Copy binary to `%LOCALAPPDATA%\Programs\kb\`, add to PATH |
| `kb init` | Interactive init (Y/n), creates `.kanban/` |
| `kb init --use-trash` | Enable trash (default) |
| `kb dashboard` | Launch web UI (serveur Rust intégré) |
| `kb add <title> -p <prio> --to <ids>` | Add task, returns UUID |
| `kb list [-p <prio>] [-s <status>]` | List tasks (excludes trash) |
| `kb move <id> <status>` | Change task status |
| `kb del <id>` | Soft delete (to trash) or hard delete |
| `kb trash [--restore <id>] [--clean-all]` | Manage trash |
| `kb config [--set key=val]` | View/set config |
| `kb data [--to-file path]` | JSON dump |
| `kb status` | KPIs |

## Dashboard Details
- Port: 5522 (auto-increment if busy)
- Serveur Rust (axum) gère fichiers statiques + API — pas de Node.js/npm
- Fichiers embarqués dans le binaire via `rust-embed` → zéro fichier à copier
- SPA fallback : les chemins inconnus servent `index.html`
- Thème dark/light, persisté dans localStorage(`kb-theme`)
- Polices : Average Sans (corps), Lily Script One (titres)
- Pas de build nécessaire — simple HTML/CSS/JS vanilla

### Dashboard Features
- 3 colonnes : À faire / En cours / Terminé
- Drag & drop cards entre colonnes (HTML5 Drag API)
- Badges priorité (low=green, medium=amber, high=rose)
- Avatars utilisateurs sur les cartes
- Ajout tâche : modal avec titre, priorité, assignation multiple
- Corbeille : FAB bottom-right, accepte drops, dialogue avec restore/clean
- Recherche instantanée (titre, personne, statut, priorité)
- Hero modal pour les détails de tâche
- Footer repliable avec liste horizontale des tâches
- Responsive (3 colonnes > 900px, 1 colonne ≤ 900px)

### API Routes
Toutes les routes sont gérées par les handlers Rust dans `src/server.rs`.
| Route | Méthode | Description |
|---|---|---|
| `/api/data` | GET | `kb data` JSON |
| `/api/move` | POST `{id, status}` | `kb move` |
| `/api/add` | POST `{title, priority, assigned_to}` | `kb add` |
| `/api/del` | POST `{id}` | `kb del` |
| `/api/folder` | GET | Retourne `{folder}` |
| `/api/init` | POST | `kb init` |
| `/api/trash-restore` | POST `{id}` | `kb trash --restore` |
| `/api/trash-clean` | POST | `kb trash --clean-all` |

## Key Design Decisions
- `.kanban/` subdir (not root) for cleanliness
- Dashboard embarqué dans le binaire — marche partout sans copier de fichiers
- `#[serde(default)]` sur `is_trash` pour backward compat
- Aucune dépendance externe pour le dashboard (HTML/CSS/JS vanilla)
- Les fichiers du dashboard sont inclus à la compilation via `rust-embed`
- Le mode `no_init_dashboard` est conservé pour compatibilité mais ne fait rien

## Version
Current: 1.2.0

## To Do
- Lier l'API réelle au dashboard (au lieu des données mock)
