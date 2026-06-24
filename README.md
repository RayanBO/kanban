# kanban cli

CLI Kanban builder with Rust + embedded HTML/CSS/JS dashboard.

🌐 Landing page: [https://rayanbo.github.io/kanban/](https://rayanbo.github.io/kanban/)

## Architecture

```text
kb (Rust CLI)
  -> .kanban/kanban.md (YAML frontmatter + Markdown tables)
  -> Rust HTTP server (axum)
  -> embedded dashboard (rust-embed, no Node/npm)
```

The dashboard is shipped inside the `kb` binary. `kb dashboard` starts the Rust server and serves both the UI assets and the JSON API.

## Storage

Project data lives in `.kanban/`:

```text
.kanban/
├── kanban.md
└── kb-config.yaml
```

## Commands

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

## Dashboard

- 3 columns: À faire / En cours / Terminé
- Drag & drop cards between columns
- Add task modal with multi-user assignment
- Mini user manager modal
- Existing task assignment edit in task detail modal
- Trash drawer + restore / clean all
- Search by title, user, status, priority
- Dark/light theme persisted in `localStorage`
- Responsive layout, no build step

## API

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

## Build

```powershell
cargo build
cargo build --release
```

## Notes

- Dashboard assets are embedded at compile time.
- `kb assign` exists for CLI parity with dashboard assignment editing.
- `delete_user` also removes deleted users from task assignments.
