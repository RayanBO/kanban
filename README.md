# kanban cli

CLI Kanban builder avec Rust. Version actuelle : v2.

## Stockage

Dossier `.kanban/` créé dans le dossier de travail courant par `kb init`.

```
.kanban/
├── kanban.md          # données (YAML frontmatter + tables Markdown)
├── kb-config.yaml     # configuration
├── dashboard.bat      # lanceur Windows
└── dashboard.sh       # lanceur Unix
```

---

## Commandes v1

```bash
# Initialiser kanban dans le dossier courant
kb init
# → crée .kanban/kanban.md + config + dashboards

# --- Tâches ---

# Ajouter une tâche
kb add "task" -p high --to "user-id-1,user-id-2,..."
# → retourne id-task

# Lister les tâches (corbeille exclue)
kb list
kb list -p high        # filtre par priorité (low | medium | high)
kb list -s done        # filtre par statut (todo | in-progress | done)

# Déplacer une tâche de statut
kb move "task-id" done

# --- Utilisateurs ---

# Créer un utilisateur
kb user add "username" --pic "path/image"
# → retourne id-user

# Modifier un utilisateur
kb user put "user-id" --username "nouveau_nom" --pic "nouveau/path"

# Supprimer un utilisateur
kb user del "user-id"

# Afficher les utilisateurs
kb user show

# --- Données ---

# KPIs globaux (corbeille exclue)
kb status

# Dump JSON dans le terminal
kb data

# Exporter JSON vers fichier
kb data --to-file path/kanban.json
```

### Priorités : `low` | `medium` | `high`
### Statuts : `todo` | `in-progress` | `done`

---

## Commandes v2

```bash
# --- Init avancé ---

# Initialiser avec options
kb init                           # mode interactif (demande confirmation)
kb init --use-trash               # activer la corbeille
kb init --use-trash=false         # désactiver la corbeille
kb init --no-init-dashboard       # sans créer les scripts dashboard

# --- Corbeille ---

# Supprimer une tâche (vers corbeille si use_trash=true)
kb del "task-id"

# Gérer la corbeille
kb trash                          # lister les tâches corbeillées
kb trash --restore "task-id"      # restaurer une tâche
kb trash --clean-all              # vider la corbeille définitivement

# --- Configuration ---

# Voir la configuration
kb config

# Modifier la configuration
kb config --set use_trash=false
kb config --set theme_dashboard=light
```

---

## Build & Install

### Dev
```powershell
cargo build                        # debug → target/debug/kb.exe
cargo run -- init                  # test sans installer
```

### Release
```powershell
cargo build --release              # optimisé → target/release/kb.exe
```

### Installer (Windows)

**Script PowerShell** — copie + PATH automatique, sans droits admin :
```powershell
powershell -ExecutionPolicy Bypass -File install.ps1
```

## à venir :

**Installeur .msi** — installeur graphique Windows :
```powershell
cargo install cargo-wix            # une seule fois
cargo wix                          # génère target/wix/kb-x.x.x-x86_64.msi
```

---

## v3

```bash
kb dashboard
# → interface web kanban sur localhost:5522
#   drag & drop, dark mode, mise à jour en temps réel
```

## v4 (à venir)

```bash
kb notif
# → notifier les users de leurs tâches assignées

kb add --agent
# → assigner une tâche à un agent code local
```
