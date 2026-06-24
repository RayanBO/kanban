# kb — Kanban CLI

**kb** is a portable Kanban board that lives in your terminal. Built with Rust, it ships as a single binary — no dependencies, no database, no cloud. When you need visuals, `kb dashboard` starts an embedded web server with a drag-and-drop board.

🌐 Landing page: [https://rayanbo.github.io/kanban/](https://rayanbo.github.io/kanban/)  
📦 License: [MIT](LICENSE)

---

## Architecture

```
kb (Rust CLI)
  → .kanban/kanban.md (YAML frontmatter + Markdown tables)
  → Rust HTTP server (axum)
  → embedded dashboard (rust-embed, no Node/npm)
```

The dashboard is shipped inside the `kb` binary. `kb dashboard` starts the Rust server and serves both the UI assets and the JSON API.

## Storage

Project data lives in `.kanban/`:

```
.kanban/
├── kanban.md
└── kb-config.yaml
```

Every task, user, and config change is written directly to disk — no external database required.

## CLI Commands

```bash
kb init [--use-trash] [--no-init-dashboard]
kb dashboard
kb add "title" -p high --to "user-id-1,user-id-2"
kb assign "task-id" --to "user-id-1,user-id-2"
kb list [-p high] [-s done]
kb move "task-id" done
kb del "task-id"
kb trash
kb trash --restore "task-id"
kb trash --clean-all
kb user add "username" [--pic "path/image"]
kb user put "user-id" [--username "new"] [--pic "new/path"]
kb user del "user-id"
kb user show
kb config
kb config --set theme_dashboard=light
kb config --set use_trash=false
kb status
kb data [--to-file path/data.json]
```

## Web Dashboard

- 3 columns: À faire / En cours / Terminé
- Drag & drop cards between columns
- Add task modal with multi-user assignment
- User manager modal (create, edit, delete)
- Inline task assignment editing from the detail view
- Trash drawer with restore and clean-all
- Search by title, user, status, priority
- Dark and light themes, persisted in `localStorage`
- Fully responsive — works on desktop and mobile

## HTTP API

| Route | Method | Purpose |
|---|---|---|
| `/api/data` | GET | Full JSON data |
| `/api/move` | POST | Change task status |
| `/api/add` | POST | Create task |
| `/api/del` | POST | Delete task |
| `/api/users` | GET/POST/PUT/DELETE | User CRUD |
| `/api/task-assign` | POST | Replace task assignees |
| `/api/folder` | GET | Current folder |
| `/api/init` | POST | Init project |
| `/api/trash-restore` | POST | Restore task from trash |
| `/api/trash-clean` | POST | Empty trash |

## Build from Source

```powershell
cargo build
cargo build --release
```

Requires Rust (install via [rustup](https://rustup.rs/)). The `kb` binary lands in `target/release/`.

## Notes

- Dashboard assets are embedded at compile time via `rust-embed`. No npm, no build step.
- `kb assign` mirrors the assignment editing available in the dashboard.
- Deleting a user also removes them from all task assignments.
- Task data is stored as a Markdown file with YAML frontmatter — version-control friendly and human readable.

---

Built by [Rayan Rav](https://rayan-rav.web.app/) · Open source under the MIT License.
