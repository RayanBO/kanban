# kb export

- Version: 1
- Exporté le: 2026-06-25T12:39:54.502521+00:00
- Tasks: 20
- Users: 1
- Comments: 0

## Config
- use_trash: false
- theme_dashboard: dark

## Users
| ID | Username | Pic |
|---|---|---|
| `798299d4-5552-49df-83bf-074216589694` | Rayan | - |

## Todo
| ID | Title | Priority | Due | Assigned | Tags | Comments |
|---|---|---|---|---|---|---|
| `bb1ba606-46b8-4615-a88c-e5ef6ccaec2a` | Ajouter un button export dans le dashboard | high | - | - | - | 0 |
| `b90fab5d-7a73-4ef7-ba5a-76d8de139f8d` | convertire le contenue des titre (task) en une contenu markdown / donc dans le conde on doit avoir une bonne contenu bien stoquer sans confusion , et ici dans dashboard on doit avoir un mini editor de markdown | high | - | - | - | 0 |
| `7d701c53-fc92-4223-934a-357fbd7e5b53` | Ajout de la Skills pour le Agent IA | high | - | Rayan | - | 0 |
| `2d5e0683-cf81-4503-a815-9f26e77fdd32` | Mettre le skills disponible dans le docs et dans une page bien precis pour ça | high | - | Rayan | - | 0 |
| `0c8e0a57-f072-4e87-8521-2d0b39c3fe31` | Ajouter de  l'acceptation de : Donate sur le Repo | high | - | Rayan | - | 0 |

## In progress
| ID | Title | Priority | Due | Assigned | Tags | Comments |
|---|---|---|---|---|---|---|
| `2760af08-099b-4ce0-94f6-54072af3797d` | Features : Export vers un vrai fichier Markdown lisible (déjà partiellement là) | medium | - | - | hello, tt | 0 |
| `9d3312b0-da8f-4b3d-a723-8045289abe31` | TODO : Changer le storage de Markdown - en - YAML . mais créer une cmd "kb export --md" qui permet d'avoir un fichier markdown (json, exel) | high | - | - | - | 0 |

## Done
| ID | Title | Priority | Due | Assigned | Tags | Comments |
|---|---|---|---|---|---|---|
| `27135178-ec5c-454e-84e4-b1920e7b32e1` | Preparer un version builder directezment dans github | medium | - | Rayan | - | 0 |
| `a6b5cfb2-fff3-48d5-9fed-48d0571e8c33` | Créer une site vitrine et déployer dans firebase , puis collé l'url dans le github repo | high | - | Rayan | - | 0 |
| `8a43ae58-bbc2-4c47-8ba8-3e5d384e069b` | wokrflow de la site vitrine | medium | - | Rayan | - | 0 |
| `04b1531f-ef24-46fc-b793-213652f23ba8` | convertire tout en englais | low | - | Rayan | - | 0 |
| `16944980-4ce8-4a10-ab45-8cff1107c5af` | à Améliorer : Le store relit le fichier à chaque requête API. Dans server.rs, chaque handler fait un store::load() indépendant. Sous faible charge c'est OK, mais une couche de state partagé avec Arc<Mutex<Store>> serait plus robuste et éviterait les I/O répétées. | high | - | - | - | 0 |
| `b17149db-fb3f-49f4-b580-b319c461a2c7` | A Améliorer : Pas de PATCH pour les tâches. Il y a /api/move et /api/task-assign mais pas de route pour éditer le titre ou la priorité d'une tâche existante depuis le dashboard. Ça force à recréer la tâche si on se trompe. | high | - | - | - | 0 |
| `bdc40b9b-218f-4327-9a51-1921341cb565` | A amlioré : Ajouter un champ optionnel due_date: Option<DateTime<Utc>> serait un ajout naturel. | high | - | - | - | 0 |
| `16cb771e-31eb-43a4-9889-ee50a4e31ed2` | A Améliorer : La recherche full-text dans kb-data.yaml : le format actuel ne permet pas de rechercher facilement via grep parce que les IDs sont tronqués dans le markdown (8 chars) mais complets dans le YAML. Ça peut créer une confusion. | high | - | - | - | 0 |
| `3beb771f-a7ff-43e2-8f76-aabbaf0be7f6` | Features : Tags/labels sur les tâches pour filtrer par catégorie | medium | - | - | - | 0 |
| `3a6bbd4e-f463-46bb-839a-93a3913ad29d` | Features : Mode watch : kb dashboard --watch qui détecte les changements fichiers et notifie le front via SSE | medium | - | - | - | 0 |
| `d5b777ba-c51b-461f-a173-654df748ba9b` | Generer mon licence | high | - | Rayan | - | 0 |
| `d69d818e-2f93-41f6-8d6f-cfa0e15610bf` | verifier le modifiable coté dashboard ! | medium | - | Rayan | - | 0 |
| `c0df1eb2-f1dc-4820-8378-475f1c3646a9` | remplacer le liens versd le copywrite en bas là dans le footer de dashboard | medium | - | - | - | 0 |

