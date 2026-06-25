---
name: kb
description: >
  Kanban CLI — portable terminal-based Kanban board with a local web dashboard.
  Manage tasks, users, tags, comments, due dates, and export data.
  Single Rust binary, zero dependencies, YAML storage.
---

# kb — Kanban CLI Agent Skill

## Quick Start

Initialize a project and start managing tasks:

```
kb init
kb add "My task" -p high --tag frontend,urgent --due 2026-12-31
kb list
kb dashboard
```

## Project State

A Kanban project lives in `.kanban/kb-data.yaml`. The store holds:

- `version` — schema version (u32)
- `config` — `use_trash` (bool), `theme_dashboard` ("dark"|"light")
- `tasks` — array of task objects
- `users` — array of user objects
- `comments` — array of comment objects

### Task fields

| Field | Type | Description |
|---|---|---|
| `id` | string | UUID (v4) |
| `title` | string | Task title (can be Markdown) |
| `priority` | "low"\|"medium"\|"high" | Priority level |
| `status` | "todo"\|"in-progress"\|"done" | Status |
| `assigned_to` | string[] | Array of user UUIDs |
| `tags` | string[] | Labels |
| `created_at` | datetime (ISO 8601) | Creation timestamp |
| `due_date` | datetime\|null | Due date |
| `is_trash` | bool | Whether task is in trash |

### User fields

| Field | Type |
|---|---|
| `id` | UUID string |
| `username` | string |
| `pic` | string\|null |
| `created_at` | datetime |

### Comment fields

| Field | Type |
|---|---|
| `id` | UUID string |
| `task_id` | UUID string (parent task) |
| `author_id` | UUID string\|null |
| `content` | string (Markdown) |
| `created_at` | datetime |
| `updated_at` | datetime\|null |

## CLI Commands

### `kb init`
Initialize a new board. Creates `.kanban/kb-data.yaml`.

```
kb init [--use-trash] [--no-init-dashboard]
```

### `kb add`
Create a task (status defaults to `todo`).

```
kb add <title> [-p <priority>] [--tag <labels>] [--to <users>] [--due <date>]
```

- `title` — positional, task content (Markdown supported)
- `-p, --priority` — `low|medium|high` (default: medium)
- `--tag` — comma-separated labels
- `--to` — comma-separated user UUIDs
- `--due` — date in `YYYY-MM-DD`

### `kb edit`
Update a task without moving it.

```
kb edit <id> [--title <text>] [-p <priority>] [--tag <labels>] [--clear-tags] [--due <date>] [--clear-due]
```

### `kb list`
List non-trashed tasks. Filter by priority, status, or tags.

```
kb list [-p <priority>] [-s <status>] [--tag <labels>]
```

### `kb move`
Change task status.

```
kb move <task_id> <new_status>
```

- `new_status` — `todo|in-progress|done`

### `kb del`
Delete a task. Moves to trash if `use_trash` is enabled.

```
kb del <task_id>
```

### `kb assign`
Replace user assignment.

```
kb assign <task_id> --to <users>
```

### `kb tags`
List all tags with usage counts.

```
kb tags
```

### `kb comment add`
Add a comment to a task.

```
kb comment add <task_id> <content> [--author-id <id>]
```

### `kb comment del`
Delete a comment by UUID.

```
kb comment del <id>
```

### `kb user`
User management subcommands:

```
kb user add <username> [--pic <path>]
kb user put <id> [--username <name>] [--pic <path>]
kb user del <id>
kb user show
```

### `kb status`
Show board KPIs (total tasks, by status, by priority, % done).

```
kb status
```

### `kb export`
Export board as JSON or Markdown.

```
kb export [--json | --md] [--to-file <path>]
```

### `kb config`
View or update config.

```
kb config [--set KEY=VALUE ...]
```

Supported keys: `use_trash`, `theme_dashboard`.

### `kb trash`
Manage trashed tasks.

```
kb trash [--restore <id>] [--clean-all]
```

### `kb dashboard`
Start the web dashboard server.

```
kb dashboard [--watch]
```

Opens at `http://localhost:<port>` (default 5522). The `--watch` flag enables file-change SSE reload.

### `kb install`
Copy binary to PATH (Windows).

## HTTP API

The dashboard server exposes a JSON API at `/api`:

| Method | Route | Purpose |
|---|---|---|
| GET | `/api/data` | Full board data |
| POST | `/api/move` | Change task status |
| POST | `/api/add` | Create task |
| POST | `/api/del` | Delete task |
| GET/POST/PUT/DELETE | `/api/users` | User CRUD |
| GET | `/api/comments/{task_id}` | List comments for a task |
| POST | `/api/comments` | Add a comment |
| DELETE | `/api/comments/{id}` | Delete a comment |
| POST | `/api/task-assign` | Replace task assignees |
| POST | `/api/task-update` | Edit task title, priority, tags, due date |
| POST | `/api/export/{format}` | Export data (json\|md) to file |
| GET | `/api/export/{format}/download` | Export data and download |
| GET | `/api/events` | SSE stream for live reload |
| GET | `/api/folder` | Current folder name |
| POST | `/api/init` | Initialize project |
| POST | `/api/trash-restore` | Restore task from trash |
| POST | `/api/trash-clean` | Empty trash |
| GET/POST | `/api/config` | Read/update config |

## Dashboard UI

The dashboard is a single-page app embedded in the binary. Features:

- 3-column board (À faire / En cours / Terminé)
- Drag & drop cards between columns
- Task detail view (hero modal) with inline edit, assignment, comments
- Task tags with click-to-filter
- Markdown task titles and comment content
- Search by title, user, status, priority, tags
- Trash drawer with restore / clean-all
- User manager (create, edit, delete)
- Export buttons (Save JSON/MD, Download JSON/MD)
- Dark/light theme (persisted in localStorage)
- Responsive layout (mobile-friendly)

## Install via Skills

```sh
npx skills add rayanbo/kanban/skills
```

Or copy this SKILL.md to your agent's skills folder (e.g. `.opencode/skills/kb/SKILL.md`).
