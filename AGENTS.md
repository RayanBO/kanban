# Kanban CLI + Dashboard — Project Context

## Overview
Kanban task manager: Rust CLI (`kb`) + Next.js web dashboard. Stockage dans `.kanban/kanban.md` (YAML frontmatter + Markdown tables).

## Architecture
```
kb (Rust CLI) ←→ .kanban/kanban.md (YAML+Markdown)
                  ↕ Rust HTTP server (axum, built into kb CLI)
                    ↕ Built static dashboard (dashboard/out/)
```
Dashboard is a Next.js static export (SSG), built once at dev time. `kb dashboard` spawns a Rust HTTP server (axum) that serves static files + handles API directly. No Node.js/npx runtime dependency.

## Project Structure
```
kanban/
├── src/
│   ├── main.rs                  # CLI entry, clap commands
│   ├── models.rs                # Task, User, Store, Config, Status, Priority
│   ├── store.rs                 # load/save kanban.md, config
│   ├── server.rs                # Rust HTTP server (axum): API + static file serving
│   └── commands/
│       ├── mod.rs
│       ├── add.rs               # kb add <title> -p <prio> --to <users>
│       ├── config.rs            # kb config --set key=val
│       ├── dashboard.rs         # kb dashboard (tokio runtime + axum server)
│       ├── data.rs              # kb data (JSON dump)
│       ├── del.rs               # kb del <id>
│       ├── init.rs              # kb init (interactive + flags)
│       ├── install.rs           # kb install (Windows PATH + dashboard copy)
│       ├── list.rs              # kb list [-p] [-s]
│       ├── move_task.rs         # kb move <id> <status>
│       ├── status.rs            # kb status (KPIs)
│       ├── trash.rs             # kb trash [--restore] [--clean-all]
│       └── user.rs              # kb user add/put/del/show
├── dashboard/                   # Next.js app (shadcn/ui, Tabler Icons) — built to out/
│   ├── app/
│   │   ├── layout.tsx           # Root layout (Geist fonts, dark support)
│   │   ├── globals.css          # Tailwind v4 + shadcn theme
│   │   └── page.tsx             # Kanban board (3 columns, drag&drop, trash)
│   ├── components/ui/           # shadcn components (badge, button, card, dialog, select, avatar)
│   ├── lib/utils.ts             # cn() utility
│   ├── public/favicon.svg       # Kanban board icon
│   └── components.json          # shadcn config (radix-mira style)
├── Cargo.toml                   # version 1.0.2
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
| `kb install` | Copy binary to `%LOCALAPPDATA%\Programs\kb\`, add to PATH, copy dashboard/ |
| `kb init` | Interactive init (Y/n), creates `.kanban/` + scripts |
| `kb init --use-trash` | Enable trash (default) |
| `kb init --no-init-dashboard` | Skip dashboard scripts |
| `kb dashboard` | Launch web UI (detects DEV/PROD mode) |
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
- Rust HTTP server (axum) handles both static files and API — no Node.js/npx at runtime
- Static files served via `service_fn` fallback, SPA fallback serves `index.html` for unmatched routes
- Background: runs in current process (no extra window), killed via process manager
- Theme: dark by default, toggle persisted in localStorage(`kb-theme`)
- Fonts: Geist (sans), Geist Mono (mono), Noto Sans (headings)
- CSS: Tailwind v4 `@theme inline` with CSS variables
- `--font-sans` maps to `--font-geist-sans` (NOT self-reference)

### Dashboard Features
- 3 columns: À faire / En cours / Terminé
- Drag & drop cards between columns (HTML5 Drag API)
- Priority badges (low=green, medium=amber, high=rose)
- User avatars on cards
- Add task dialog with title, priority, user assignment
- Delete card (to trash)
- Trash bin: FAB bottom-right, accepts drops, dialog with restore/clean
- Not-initialized screen with init button
- Page title: `Kanban {folder}` (dynamic)
- Favicon: SVG kanban board icon

### API Routes
All routes are handled by Rust handlers in `src/server.rs` (no exec/subprocess).
| Route | Method | Description |
|---|---|---|
| `/api/data` | GET | `kb data` JSON |
| `/api/move` | POST `{id, status}` | `kb move` |
| `/api/add` | POST `{title, priority, assigned_to}` | `kb add` |
| `/api/del` | POST `{id}` | `kb del` |
| `/api/folder` | GET | Returns `{folder}` name |
| `/api/init` | POST | `kb init --use-trash=true --no-init-dashboard` |
| `/api/trash-restore` | POST `{id}` | `kb trash --restore` |
| `/api/trash-clean` | POST | `kb trash --clean-all` |

## Key Design Decisions
- `.kanban/` subdir (not root) for cleanliness
- `#[serde(default)]` on `is_trash` for backward compat
- Banner uses Unicode box drawing (no ANSI codes) for multi-terminal reliability
- Logo KANBAN in ASCII blocks (█) — no emoji in logo
- Padding uses `chars().take(w)` for safe Unicode (no byte slicing)
- `emoji_w()` handles U+1F000–U+1FFFF, U+2699, U+2705, U+2795, U+2796 as double-width
- Dashboard dir: checks `./dashboard/` first, then `<install_dir>/dashboard/`
- PROD build auto-runs `npx next build` if `.next/` missing
- Dashboard scripts (`.kanban/dashboard.bat`/`.sh`) launch `kb dashboard`

## Version
Current: 1.0.2

## To Do
- `kb notif` — notification for assigned users

## Version
Current: 1.0.2

## To Do
- `kb notif` — notification for assigned users
