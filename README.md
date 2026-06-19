# kanban cli

CLI Kanban builder avec Rust + Dashboard web Next.js.

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                    kb (Rust CLI)                     │
│  init · add · list · move · del · trash · dashboard │
└──────────┬──────────────────────────────────────────┘
           │ lit/écrit
           ▼
┌────────────────────┐
│  .kanban/          │ ← stockage fichier unique
│  └── kanban.md     │   (YAML + Markdown)
└────────────────────┘
           ▲
┌──────────┴──────────────────────────────────────────┐
│            Dashboard (Next.js + shadcn)              │
│  Page Kanban · API routes → exec kb CLI             │
│  Dark/light · Drag & drop · Corbeille               │
└─────────────────────────────────────────────────────┘
```

Le CLI écrit les données dans `.kanban/kanban.md` (frontmatter YAML + tables Markdown).
Le dashboard Next.js lit/écrit via les API routes qui appellent le binaire `kb`.

---

## Stockage

Dossier `.kanban/` créé dans le dossier courant par `kb init`.

```
.kanban/
├── kanban.md          # données (YAML frontmatter + tables Markdown)
├── kb-config.yaml     # configuration
├── dashboard.bat      # lanceur Windows → kb dashboard
└── dashboard.sh       # lanceur Unix → kb dashboard
```

---

## Commandes

```bash
# --- Installation ---

# Installer kb dans le PATH (Windows)
kb install
# → copie le binaire + ajoute au PATH utilisateur
# Ouvre un nouveau terminal après installation

# --- Initialisation ---

kb init                           # interactif (Y/n, Enter accepte)
kb init --use-trash               # activer la corbeille (défaut: true)
kb init --use-trash=false         # désactiver la corbeille
kb init --no-init-dashboard       # sans scripts dashboard

# --- Dashboard ---

kb dashboard                      # interface web localhost:5522
# DEV  (cargo run)     → npx next dev   (hot reload)
# PROD (binaire installé) → npx next start (build auto si manquant)

# --- Tâches ---

# Ajouter une tâche
kb add "title" -p high --to "user-id-1,user-id-2"
# → retourne id-task

# Lister (corbeille exclue)
kb list
kb list -p high        # filtre priorité (low | medium | high)
kb list -s done        # filtre statut (todo | in-progress | done)

# Changer le statut
kb move "task-id" done

# Supprimer (→ corbeille si use_trash=true)
kb del "task-id"

# --- Corbeille ---

kb trash                          # lister
kb trash --restore "task-id"      # restaurer
kb trash --clean-all              # vider définitivement

# --- Utilisateurs ---

kb user add "username" --pic "path/image"
# → retourne id-user

kb user put "user-id" --username "new" --pic "new/path"
kb user del "user-id"
kb user show

# --- Configuration ---

kb config                         # voir
kb config --set use_trash=false
kb config --set theme_dashboard=light

# --- Données ---

kb status                         # KPIs (corbeille exclue)
kb data                           # dump JSON
kb data --to-file path/data.json  # export JSON

```

### Priorités : `low` | `medium` | `high`
### Statuts : `todo` | `in-progress` | `done`

---

## Dashboard

Le dashboard est une app Next.js (dans `dashboard/`) avec **shadcn/ui** et **Tabler Icons**.

| Fonctionnalité | Détail |
|---|---|
| Kanban 3 colonnes | À faire / En cours / Terminé |
| Drag & drop | Glisser une carte entre colonnes |
| Badges priorité | low (vert) · medium (ambre) · high (rose) |
| Assignation | Avatars utilisateurs sur les cartes |
| Corbeille | Drop une carte → corbeille, restaurer, vider |
| Dark/Light | Toggle persistant dans localStorage |
| Ajout rapide | Dialog avec titre, priorité, assignés |

Le dashboard détecte automatiquement le mode :

- **DEV** : binaire dans `target/debug/` → `npx next dev` (hot reload)
- **PROD** : binaire installé → `npx next start` (build auto si `.next/` absent)

---

## Build & Install

```powershell
cargo build                  # debug → target/debug/kb.exe
cargo build --release        # release → target/release/kb.exe
```

**Installation (Windows)** — 3 façons :

| Méthode | Commande |
|---|---|
| Double-clic | `target/release/kb.exe` (auto-install) |
| Terminal | `target/release/kb.exe install` |
| Script | `powershell -ExecutionPolicy Bypass -File install.ps1` |

Après installation, ouvre un **nouveau terminal** et tape `kb --version`.

---

## À venir

```bash
kb notif              # notification users assignés
```
