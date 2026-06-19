# Kanban Dashboard v2

Kanban board pure HTML + CSS + vanilla JS inline. Zéro dépendance, zéro framework.

## Features

- **3 colonnes :** À faire / En cours / Terminé
- **Drag & drop** entre colonnes (HTML5 Drag API)
- **Ajout de tâche** avec titre, priorité, assignation multiple
- **Corbeille** — drop direct sur le FAB, restauration, vidage
- **Footer tâches** — panneau coulissant avec scroll horizontal, cartes triées par priorité
- **Modal hero** — vue détaillée des tâches avec badges et avatars
- **Thème Dark/Light** — bouton lune/soleil, persisté dans localStorage
- **Responsive** — 3 colonnes > 900px, 1 colonne ≤ 900px
- **Données mock** — 11 tâches pré-chargées, 4 utilisateurs

## Utilisation

Ouvre `index.html` dans un navigateur.

```bash
start index.html
# ou
python -m http.server 8000
# puis http://localhost:8000
```

## Structure

```
├── index.html     # Structure HTML + JS (drag & drop, modals, state)
├── style.css      # Styles complets (thèmes, responsive, animations)
├── fonts/         # Self-hosted woff2
└── AGENTS.md      # Contexte projet pour agents IA
```

## Tech

| Aspect | Détail |
|--------|--------|
| Stack | HTML5, CSS3, Vanilla JS (ES6) |
| Dépendances | Aucune |
| Icônes | Inline SVG |
| Font | Average Sans (corps), Lily Script One (titres) — self-hosted woff2 |
| Thème | CSS custom properties, `[data-theme]` |
