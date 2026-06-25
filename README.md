# kb — Kanban CLI

**kb** is a portable Kanban board that lives in your terminal. Built with Rust, it ships as a single binary — no dependencies, no database, no cloud. When you need visuals, `kb dashboard` starts an embedded web server with a drag-and-drop board, task tags, and live reload support.

🌐 Landing page: [https://rayanbo.github.io/kanban/](https://rayanbo.github.io/kanban/)  
📦 License: [MIT](LICENSE)

---

## Architecture

```
kb (Rust CLI)
  → .kanban/kb-data.yaml (tasks, users, config, comments)
  → Rust HTTP server (axum)
  → embedded dashboard (rust-embed, no Node/npm)
```

The dashboard is shipped inside the `kb` binary. `kb dashboard` starts the Rust server and serves both the UI assets and the JSON API.

## Storage

Project data lives in `.kanban/`:

```
.kanban/
└── kb-data.yaml
```

Every task, user, and config change is written directly to disk — no external database required.

## CLI Commands

```bash
kb init [--use-trash] [--no-init-dashboard]
kb dashboard [--watch]
kb add "title" -p high --tag backend,urgent --to "user-id-1,user-id-2"
kb assign "task-id" --to "user-id-1,user-id-2"
kb edit "task-id" --tag backend,urgent --clear-due
kb list [-p high] [-s done] [--tag backend,urgent]
kb tags
kb move "task-id" done
kb del "task-id"
kb trash
kb trash --restore "task-id"
kb trash --clean-all
kb user add "username" [--pic "path/image"]
kb user put "user-id" [--username "new"] [--pic "new/path"]
kb user del "user-id"
kb user show
kb comment add "task-id" "Comment text" --author-id "user-id"
kb comment del "comment-id"
kb config
kb config --set theme_dashboard=light
kb config --set use_trash=false
kb status
kb export --json [--to-file path/output.json]
kb export --md [--to-file path/output.md]
kb data (legacy alias for `kb export --json`)
```

## Web Dashboard

- 3 columns: À faire / En cours / Terminé
- Drag & drop cards between columns
- Add task modal with multi-user assignment and labels
- User manager modal (create, edit, delete)
- Task detail view with inline assignment editing
- Task edit modal for title, priority, tags, and due date
- Trash drawer with restore and clean-all
- Search by title, user, status, priority, and tags
- Export buttons for JSON and Markdown backups, plus browser download
- Dark and light themes, persisted in `localStorage`
- `kb dashboard --watch` for file-change reloads via SSE
- Fully responsive — works on desktop and mobile

## HTTP API

| Route | Method | Purpose |
|---|---|---|
| `/api/data` | GET | Full JSON data |
| `/api/move` | POST | Change task status |
| `/api/add` | POST | Create task |
| `/api/del` | POST | Delete task |
| `/api/users` | GET/POST/PUT/DELETE | User CRUD |
| `/api/comments/{task_id}` | GET | List comments for a task |
| `/api/comments` | POST | Add a comment |
| `/api/comments/{id}` | DELETE | Delete a comment |
| `/api/task-assign` | POST | Replace task assignees |
| `/api/task-update` | POST | Edit task title, priority, tags, or due date |
| `/api/export/{format}` | POST | Export board data to `.kanban/kb-export.json` or `.md` |
| `/api/export/{format}/download` | GET | Export and download the file through the browser |
| `/api/events` | GET | SSE reload stream for dashboard watch mode |
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
- `kb edit` and the dashboard task modal can update tags and due dates.
- `kb list --tag ...` and the dashboard tag chips filter tasks by labels.
- `kb export --json` and `kb export --md` write readable backups in `.kanban/`.
- `kb tags` prints the label inventory with usage counts.
- `kb dashboard --watch` reloads the UI when `.kanban/kb-data.yaml` changes.
- Deleting a user also removes them from all task assignments.
- Task data is stored as strict YAML — version-control friendly and easy to extend with comments.

---

Built by [Rayan Rav](https://rayan-rav.web.app/) · Open source under the MIT License.
