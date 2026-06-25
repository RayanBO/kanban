---
tasks:
- id: 27135178-ec5c-454e-84e4-b1920e7b32e1
  title: Preparer un version builder directezment dans github
  priority: medium
  status: done
  assigned_to:
  - 798299d4-5552-49df-83bf-074216589694
  created_at: 2026-06-24T14:50:23.714575200Z
  due_date: null
  is_trash: false
- id: a6b5cfb2-fff3-48d5-9fed-48d0571e8c33
  title: CrÃ©er une site vitrine et dÃ©ployer dans firebase , puis collÃ© l'url dans le github repo
  priority: high
  status: done
  assigned_to:
  - 798299d4-5552-49df-83bf-074216589694
  created_at: 2026-06-24T14:51:22.538656900Z
  due_date: null
  is_trash: false
- id: 8a43ae58-bbc2-4c47-8ba8-3e5d384e069b
  title: wokrflow de la site vitrine
  priority: medium
  status: done
  assigned_to:
  - 798299d4-5552-49df-83bf-074216589694
  created_at: 2026-06-24T14:53:00.136924300Z
  due_date: null
  is_trash: false
- id: 04b1531f-ef24-46fc-b793-213652f23ba8
  title: convertire tout en englais
  priority: low
  status: done
  assigned_to:
  - 798299d4-5552-49df-83bf-074216589694
  created_at: 2026-06-24T15:11:46.136653700Z
  due_date: null
  is_trash: false
- id: 16944980-4ce8-4a10-ab45-8cff1107c5af
  title: 'Ã  AmÃ©liorer : Le store relit le fichier Ã  chaque requÃªte API. Dans server.rs, chaque handler fait un store::load() indÃ©pendant. Sous faible charge c''est OK, mais une couche de state partagÃ© avec Arc<Mutex<Store>> serait plus robuste et Ã©viterait les I/O rÃ©pÃ©tÃ©es.'
  priority: high
  status: done
  assigned_to: []
  created_at: 2026-06-24T15:13:45.751990900Z
  due_date: null
  is_trash: false
- id: b17149db-fb3f-49f4-b580-b319c461a2c7
  title: 'A AmÃ©liorer : Pas de PATCH pour les tÃ¢ches. Il y a /api/move et /api/task-assign mais pas de route pour Ã©diter le titre ou la prioritÃ© d''une tÃ¢che existante depuis le dashboard. Ã‡a force Ã  recrÃ©er la tÃ¢che si on se trompe.'
  priority: high
  status: done
  assigned_to: []
  created_at: 2026-06-24T15:14:04.980531Z
  due_date: null
  is_trash: false
- id: bdc40b9b-218f-4327-9a51-1921341cb565
  title: 'A amliorÃ© : Ajouter un champ optionnel due_date: Option<DateTime<Utc>> serait un ajout naturel.'
  priority: high
  status: done
  assigned_to: []
  created_at: 2026-06-24T15:14:51.521876900Z
  due_date: null
  is_trash: false
- id: 16cb771e-31eb-43a4-9889-ee50a4e31ed2
  title: 'A AmÃ©liorer : La recherche full-text dans kanban.md : le format actuel ne permet pas de rechercher facilement via grep parce que les IDs sont tronquÃ©s dans le markdown (8 chars) mais complets dans le YAML. Ã‡a peut crÃ©er une confusion.'
  priority: high
  status: in-progress
  assigned_to: []
  created_at: 2026-06-24T15:15:40.893774Z
  due_date: 2026-06-18T00:00:00Z
  is_trash: false
- id: 3beb771f-a7ff-43e2-8f76-aabbaf0be7f6
  title: 'Features : Tags/labels sur les tÃ¢ches pour filtrer par catÃ©gorie'
  priority: medium
  status: todo
  assigned_to: []
  created_at: 2026-06-24T15:16:09.125727900Z
  due_date: null
  is_trash: false
- id: 2760af08-099b-4ce0-94f6-54072af3797d
  title: 'Features : Export vers un vrai fichier Markdown lisible (dÃ©jÃ  partiellement lÃ )'
  priority: medium
  status: todo
  assigned_to: []
  created_at: 2026-06-24T15:16:43.929903700Z
  due_date: null
  is_trash: false
- id: 3a6bbd4e-f463-46bb-839a-93a3913ad29d
  title: 'Features : Mode watch : kb dashboard --watch qui dÃ©tecte les changements fichiers et notifie le front via SSE'
  priority: medium
  status: todo
  assigned_to: []
  created_at: 2026-06-24T15:17:09.364346500Z
  due_date: null
  is_trash: false
- id: 9d3312b0-da8f-4b3d-a723-8045289abe31
  title: 'TODO : Changer le storage de Markdown - en - YAML . mais crÃ©er une cmd "kb export --md" qui permet d''avoir un fichier markdown (json, exel)'
  priority: high
  status: todo
  assigned_to: []
  created_at: 2026-06-24T15:21:49.318381300Z
  due_date: null
  is_trash: false
- id: bb1ba606-46b8-4615-a88c-e5ef6ccaec2a
  title: Ajouter un button export dans le dashboard
  priority: high
  status: todo
  assigned_to: []
  created_at: 2026-06-24T15:22:54.235042300Z
  due_date: null
  is_trash: false
- id: d5b777ba-c51b-461f-a173-654df748ba9b
  title: Generer mon licence
  priority: high
  status: done
  assigned_to:
  - 798299d4-5552-49df-83bf-074216589694
  created_at: 2026-06-24T15:23:43.996573200Z
  due_date: null
  is_trash: false
- id: c99504c2-b52a-4cba-b770-052c46d927ff
  title: CrÃ©er une version builder en msi + exe / ajouter une possibilitÃ© de installer sans refus de windows defender
  priority: high
  status: todo
  assigned_to:
  - 798299d4-5552-49df-83bf-074216589694
  created_at: 2026-06-24T15:26:19.530868400Z
  due_date: null
  is_trash: false
- id: b90fab5d-7a73-4ef7-ba5a-76d8de139f8d
  title: convertire le contenue des titre (task) en une contenu markdown / donc dans le conde on doit avoir une bonne contenu bien stoquer sans confusion , et ici dans dashboard on doit avoir un mini editor de markdown
  priority: high
  status: todo
  assigned_to: []
  created_at: 2026-06-24T15:27:24.429661400Z
  due_date: null
  is_trash: false
- id: 7d701c53-fc92-4223-934a-357fbd7e5b53
  title: Ajout de la Skills pour le Agent IA
  priority: high
  status: todo
  assigned_to:
  - 798299d4-5552-49df-83bf-074216589694
  created_at: 2026-06-24T18:33:55.501471700Z
  due_date: null
  is_trash: false
- id: 2d5e0683-cf81-4503-a815-9f26e77fdd32
  title: Mettre le skills disponible dans le docs et dans une page bien precis pour Ã§a
  priority: high
  status: todo
  assigned_to:
  - 798299d4-5552-49df-83bf-074216589694
  created_at: 2026-06-24T18:34:22.976160800Z
  due_date: null
  is_trash: false
- id: 0c8e0a57-f072-4e87-8521-2d0b39c3fe31
  title: 'Ajouter de  l''acceptation de : Donate sur le Repo'
  priority: high
  status: todo
  assigned_to:
  - 798299d4-5552-49df-83bf-074216589694
  created_at: 2026-06-25T03:09:58.473888200Z
  due_date: null
  is_trash: false
- id: d69d818e-2f93-41f6-8d6f-cfa0e15610bf
  title: verifier le modifiable cotÃ© dashboard !
  priority: medium
  status: done
  assigned_to:
  - 798299d4-5552-49df-83bf-074216589694
  created_at: 2026-06-25T05:31:53.723496600Z
  due_date: null
  is_trash: false
- id: c0df1eb2-f1dc-4820-8378-475f1c3646a9
  title: remplacer le liens versd le copywrite en bas lÃ  dans le footer de dashboard
  priority: medium
  status: done
  assigned_to: []
  created_at: 2026-06-25T05:32:13.820592200Z
  due_date: null
  is_trash: false
users:
- id: 798299d4-5552-49df-83bf-074216589694
  username: Rayan
  pic: null
  created_at: 2026-06-24T14:50:29.612563200Z
---

# Kanban

## Users

| ID | Username | Pic |
|----|----------|-----|
| `798299d4-5552-49df-83bf-074216589694` | Rayan | - |

## Tasks

### Todo

| ID | Title | Priority | Due | Assigned |
|----|-------|----------|-----|----------|
| `3beb771f-a7ff-43e2-8f76-aabbaf0be7f6` | Features : Tags/labels sur les tÃ¢ches pour filtrer par catÃ©gorie | medium | - | - |
| `2760af08-099b-4ce0-94f6-54072af3797d` | Features : Export vers un vrai fichier Markdown lisible (dÃ©jÃ  partiellement lÃ ) | medium | - | - |
| `3a6bbd4e-f463-46bb-839a-93a3913ad29d` | Features : Mode watch : kb dashboard --watch qui dÃ©tecte les changements fichiers et notifie le front via SSE | medium | - | - |
| `9d3312b0-da8f-4b3d-a723-8045289abe31` | TODO : Changer le storage de Markdown - en - YAML . mais crÃ©er une cmd "kb export --md" qui permet d'avoir un fichier markdown (json, exel) | high | - | - |
| `bb1ba606-46b8-4615-a88c-e5ef6ccaec2a` | Ajouter un button export dans le dashboard | high | - | - |
| `c99504c2-b52a-4cba-b770-052c46d927ff` | CrÃ©er une version builder en msi + exe / ajouter une possibilitÃ© de installer sans refus de windows defender | high | - | Rayan |
| `b90fab5d-7a73-4ef7-ba5a-76d8de139f8d` | convertire le contenue des titre (task) en une contenu markdown / donc dans le conde on doit avoir une bonne contenu bien stoquer sans confusion , et ici dans dashboard on doit avoir un mini editor de markdown | high | - | - |
| `7d701c53-fc92-4223-934a-357fbd7e5b53` | Ajout de la Skills pour le Agent IA | high | - | Rayan |
| `2d5e0683-cf81-4503-a815-9f26e77fdd32` | Mettre le skills disponible dans le docs et dans une page bien precis pour Ã§a | high | - | Rayan |
| `0c8e0a57-f072-4e87-8521-2d0b39c3fe31` | Ajouter de  l'acceptation de : Donate sur le Repo | high | - | Rayan |

### In Progress

| ID | Title | Priority | Due | Assigned |
|----|-------|----------|-----|----------|
| `16cb771e-31eb-43a4-9889-ee50a4e31ed2` | A AmÃ©liorer : La recherche full-text dans kanban.md : le format actuel ne permet pas de rechercher facilement via grep parce que les IDs sont tronquÃ©s dans le markdown (8 chars) mais complets dans le YAML. Ã‡a peut crÃ©er une confusion. | high | 2026-06-18 | - |

### Done

| ID | Title | Priority | Due | Assigned |
|----|-------|----------|-----|----------|
| `27135178-ec5c-454e-84e4-b1920e7b32e1` | Preparer un version builder directezment dans github | medium | - | Rayan |
| `a6b5cfb2-fff3-48d5-9fed-48d0571e8c33` | CrÃ©er une site vitrine et dÃ©ployer dans firebase , puis collÃ© l'url dans le github repo | high | - | Rayan |
| `8a43ae58-bbc2-4c47-8ba8-3e5d384e069b` | wokrflow de la site vitrine | medium | - | Rayan |
| `04b1531f-ef24-46fc-b793-213652f23ba8` | convertire tout en englais | low | - | Rayan |
| `16944980-4ce8-4a10-ab45-8cff1107c5af` | Ã  AmÃ©liorer : Le store relit le fichier Ã  chaque requÃªte API. Dans server.rs, chaque handler fait un store::load() indÃ©pendant. Sous faible charge c'est OK, mais une couche de state partagÃ© avec Arc<Mutex<Store>> serait plus robuste et Ã©viterait les I/O rÃ©pÃ©tÃ©es. | high | - | - |
| `b17149db-fb3f-49f4-b580-b319c461a2c7` | A AmÃ©liorer : Pas de PATCH pour les tÃ¢ches. Il y a /api/move et /api/task-assign mais pas de route pour Ã©diter le titre ou la prioritÃ© d'une tÃ¢che existante depuis le dashboard. Ã‡a force Ã  recrÃ©er la tÃ¢che si on se trompe. | high | - | - |
| `bdc40b9b-218f-4327-9a51-1921341cb565` | A amliorÃ© : Ajouter un champ optionnel due_date: Option<DateTime<Utc>> serait un ajout naturel. | high | - | - |
| `d5b777ba-c51b-461f-a173-654df748ba9b` | Generer mon licence | high | - | Rayan |
| `d69d818e-2f93-41f6-8d6f-cfa0e15610bf` | verifier le modifiable cotÃ© dashboard ! | medium | - | Rayan |
| `c0df1eb2-f1dc-4820-8378-475f1c3646a9` | remplacer le liens versd le copywrite en bas lÃ  dans le footer de dashboard | medium | - | - |

