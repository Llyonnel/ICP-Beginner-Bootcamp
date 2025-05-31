# Rust Task Manager

Un gestionnaire de tâches en ligne de commande écrit en Rust. Il permet d'ajouter, afficher, compléter, supprimer, sauvegarder et charger des tâches.

## Fonctionnalités

- Ajouter une tâche avec un titre, une description et une date d'échéance optionnelle
- Afficher toutes les tâches avec leurs statuts
- Marquer une tâche comme complétée
- Supprimer une tâche
- Sauvegarder les tâches dans un fichier JSON
- Charger les tâches depuis un fichier JSON
- Filtrage possible

## Installation

Assurez-vous d'avoir [Rust installé](https://www.rust-lang.org/tools/install).

Clonez ce dépôt et compilez le projet :

```bash
git clone https://github.com/...
cd <nom du dossier>
cargo build --release
```

## Lancer le programme

```bash
cargo run main.rs (ici --bin module5_final)
```

## Commandes

| Commande                        | Description                                |
| ------------------------------- | ------------------------------------------ |
| `add <titre> <desc> <échéance>` | Ajoute une tâche                           |
| `list`                          | Affiche toutes les tâches                  |
| `complete <id>`                 | Marque la tâche comme complétée            |
| `delete <id>`                   | Supprime la tâche                          |
| `save <nom_fichier>`            | Sauvegarde les tâches dans un fichier JSON |
| `load <nom_fichier>`            | Charge les tâches depuis un fichier JSON   |
| `filter <status>`               | Filtre les tâches par statut               |
| `quit` ou `exit`                | Quitte l'application                       |

## Exemple

add Apprendre_Rust Lire_le_livre 2025-06-01  
list  
complete 1  
save tasks.json  
load tasks.json  
delete 1  
filter pending  
quit
