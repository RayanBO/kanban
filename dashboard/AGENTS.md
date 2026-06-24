# Kanban Dashboard v2 — Project Context

**IMPORTANT: Keep this file updated.** Every agent working on this project MUST update AGENTS.md when adding/removing features, changing architecture, or modifying design decisions.

## Overview
Pure HTML + CSS + vanilla JS kanban board. Zero dependencies. Open in browser directly. State comes from Rust API at runtime, not mock data.

## Data Model (JavaScript)
```js
Task { id, title, priority, status, assigned }
User { id, username, pic, color }
```

- `priority`: `low` | `medium` | `high`
- `status`: `todo` | `in-progress` | `done`
- `assigned`: array of user IDs

## Features

### Columns
- 3 colonnes: À faire / En cours / Terminé
- Compteur de tâches par colonne
- État vide "Aucune tâche" si colonne vide

### Drag & Drop
- HTML5 Drag API natif
- Cards `draggable="true"`
- `dragstart` / `dragend` / `dragover` / `dragenter` / `dragleave` / `drop`
- `.dragging` class sur carte traînée (opacity réduite)
- `.drag-over` class sur colonne survolée (bordure accent)

### Trash (Corbeille)
- FAB bouton fixe en bas à droite
- Click → ouvre modal corbeille
- Drag & drop d'une carte directement sur le FAB
- Animation : couvercle rotate 22°, scale 1.15, couleur rose
- Drop → animation "swallow" (scale 1.35 puis rebond)
- Modal : lister tâches supprimées, restaurer, vider
- Delete définitif depuis la modale

### Add Task
- Modal avec formulaire : titre, priorité, assignation multiple
- Validation basique (titre requis)
- Nouvelle tâche créée en statut `todo`

### User Manager
- Modal compact pour ajouter / modifier / supprimer utilisateurs
- Assignation des users via select multiple dans le hero modal
- Ajout tâche garde assignation multiple existante

### Search (Recherche)
- Barre de recherche dans le header avec icône loupe
- Focus → ouvre un overlay de résultats (mode hero)
- Filtrage instantané par titre, utilisateur assigné, statut, priorité
- Résultats affichent : titre (avec highlight), badge priorité, statut, avatars
- Click sur un résultat → ouvre le hero modal de la tâche
- Close via Escape, click overlay, ou blur avec délai
- Responsive : s'adapte sur mobile

### Theme (Dark/Light)
- CSS custom properties (variables) dans `:root` et `[data-theme="light"]`
- Toggle bouton lune/soleil dans header
- Animation rotation 3D sur l'icône
- Persisté dans `localStorage('kb-theme')`
- Respecte `prefers-color-scheme` système
- Transition fluide 0.3s via classe `.theme-transitioning`

### Footer (Collapsible Task Panel)
- Toggle bar always visible at bottom of screen
- Click → slide-up panel with horizontal scrollable task list
- Cards sorted by priority: high → medium → low (left to right)
- Each card: fixed 220px width, multi-line title wraps up to 3 lines
- Card layout: header (priority dot+label left, status icon right) / body (title) / mini-footer (user avatars)
- Status icons: ◯ (À faire), ◐ (En cours, accent), ✓ (Terminé, green)
- User avatars: 20px overlapping circles with color initials
- Click card → opens hero modal with full details
- Updates reactively when tasks change (add, move, trash)
- Layout: flex column, board shrinks when footer opens
- **Content Reveal Effect**: Cards blank at rest (just background/border). On hover → content fades in via `--r` CSS var:
  - Hovered card: `--r: 1` (full content)
  - 1st sibling: `--r: 0.6` (partial)
  - 2nd sibling: `--r: 0.25` (faint)
  - 3rd+ sibling: `--r: 0` (hidden)
  - Controlled by `opacity: var(--r); transition: opacity 0.3s` on `.ft-header, .ft-body, .ft-users`

### Hero Modal
- Large overlay modal for task detail view
- Shows title (large, heading font), status badge, priority badge
- Shows assigned users with avatar pills
- Lets user reassign task via multi-select + save
- Close via × button, overlay click, or Escape

### Responsive
| Breakpoint | Behavior |
|---|---|
| > 900px | 3 colonnes côte à côte |
| ≤ 900px | 1 colonne, body scrollable |
| ≤ 600px | Header compact, label bouton caché, FAB réduit, footer compact |

## Design System
| Token | Dark | Light |
|---|---|---|
| `--bg` | `#0b0b0e` | `#f2f2f5` |
| `--surface` | `#141417` | `#ffffff` |
| `--surface-elevated` | `#1c1c21` | `#ebebef` |
| `--border` | `#26262b` | `#d9d9df` |
| `--text` | `#f4f4f5` | `#1c1c1e` |
| `--accent` | `#6366f1` | `#6366f1` |
| `--rose` | `#ef4444` | `#ef4444` |
| `--green` | `#22c55e` | `#22c55e` |
| `--amber` | `#f59e0b` | `#f59e0b` |

- Radius: 6px (small), 10px (default), 14px (large)
- Font: **Average Sans** (corps, `--font`), **Lily Script One** (titres, `--font-heading`)
- Fonts locales dans `fonts/` (woff2, `font-display: swap`)
- Shadows: dark uses heavy black, light uses light black

## Accessibility
- `:focus-visible` sur tous les contrôles interactifs
- `aria-label` sur boutons icônes
- Escape key ferme les modales
- Labels `<label>` associés aux inputs

## Performance
- Aucune dépendance externe
- CSS < 15 KB, HTML < 15 KB
- Inline SVG icons (zero HTTP requests)
- Pas de reflows inutiles (transform/opacity only)

## Conventions
- BEM-lite naming (classes descriptives, `.card`, `.card-title`, `.card-actions`)
- CSS variables pour toutes les couleurs
- Transition 0.15s pour interactions, 0.2-0.3s pour thème
- French UI labels (À faire, En cours, Terminé, Corbeille)
- snake_case IDs, kebab-case CSS classes
- Functions déclarées (hoisting OK)
- `let` plutôt que `const` pour le JS

## To Do
- [ ] `prefers-reduced-motion` support
- [ ] Édition complète de tâche
- [ ] Filtres / recherche avancés
- [ ] Reordering dans une colonne
- [ ] Animations entrée/sortie cartes
