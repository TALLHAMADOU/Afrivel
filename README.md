<div align="center">

# Afrivel

**L'expérience Laravel, propulsée par Rust.**

Un framework web full-stack, *batteries-included*, qui offre la productivité de Laravel
avec la performance, la sécurité mémoire et la concurrence de Rust.

[Documentation](./docs/README.md) · [Roadmap](./docs/ROADMAP.md) · [Contribuer](#contribuer)

</div>

---

> ⚠️ **Statut : v0.0.1 en cours de design.** Le framework n'est pas encore utilisable. Cette page décrit l'expérience cible.

## Pourquoi Afrivel ?

Rust dispose d'excellentes briques web (Axum, Actix, SeaORM). Mais il manque un framework
**full-stack** offrant une expérience aussi fluide que Laravel : conventions claires, génération
de code, architecture modulaire dès le premier jour.

Afrivel comble ce manque — sans réinventer ce que l'écosystème Rust fait déjà très bien.
Il s'appuie sur **Axum + Tower + Tokio** et **SeaORM**, et ajoute la couche de productivité.

**Ce qui distingue Afrivel** : une approche **module-centric** (chaque fonctionnalité est une
tranche verticale autonome) avec une **architecture en couches par défaut** — Services,
Repositories, Interfaces, Resources. Pensé pour des applications **maintenables, en équipe,
à grande échelle**, là où d'autres frameworks visent surtout le prototypage rapide.

## Philosophie

- **Productivité avant tout** — concentre-toi sur la logique métier, pas sur la configuration.
- **Convention over configuration** — les bonnes pratiques par défaut.
- **Architecture modulaire** — chaque fonctionnalité est un module autonome.
- **Génération de code intelligente** — une commande génère une fonctionnalité complète.
- **Performance native** — toute la puissance de Rust, sans sacrifier l'expérience développeur.
- **Explicite, pas magique** — aucune réflexion runtime ; tout est généré et lisible.

## La fonctionnalité signature : les modules

Au lieu de générer des composants isolés, Afrivel raisonne en **modules métier complets**.

```bash
afrivel make:module Auth
```

génère un module **totalement autonome** :

```text
modules/Auth/
├── mod.rs            # enregistre routes & services du module
├── models/      requests/      controllers/   services/
├── interfaces/  repositories/  resources/
├── migrations/  routes/        tests/
```

## Génération CRUD automatique

À partir d'une simple définition de modèle, Afrivel génère **tout** le CRUD :

```bash
afrivel make:module User \
    --model User:name:string,email:string:unique,password:string
```

Produit : migrations, modèle, DTOs, validateurs de requêtes, resources, services,
repositories, interfaces, contrôleurs, routes et tests — **toujours compilables**.

## Structure d'un projet

```text
myapp/
├── Afrivel.toml      # manifeste du projet
├── src/              # bootstrap (main.rs) + binaire CLI délégué
├── modules/          # tes modules métier autonomes
├── config/  routes/  database/  storage/  tests/
```

## Boucle de développement

```bash
afrivel dev      # watch → recompile → restart (le serveur reste vivant si le build échoue)
afrivel migrate  # applique les migrations (SeaORM)
afrivel serve    # lance l'application
```

## Aperçu des commandes

| Commande | Rôle |
|----------|------|
| `afrivel new <nom>` | Crée un nouveau projet. |
| `afrivel make:module <Nom> [--model …]` | Génère un module complet (+ CRUD). |
| `afrivel make:model\|controller\|service\|…` | Génère un composant granulaire. |
| `afrivel migrate` / `migrate:rollback` / `migrate:status` | Migrations. |
| `afrivel db:seed` | Données de test. |
| `afrivel dev` / `serve` | Développement / production. |
| `afrivel module:list` / `route:list` | Introspection. |
| `afrivel completion <shell>` | Complétion shell. |

## Fonctionnalités

**v0.0.1 (en cours)**
- Routing, middleware, configuration, logging, validation, DI léger
- ORM ergonomique (CRUD + relations) au-dessus de SeaORM, migrations, factories, seeders (Postgres)
- CLI `afrivel` : `new`, `make:module`, génération CRUD, migrations, dev loop
- Module **Auth** : JWT, RBAC, permissions, hashing Argon2

**Prévu ensuite**
OAuth2 · Jobs/Queues/Events/Scheduler · API versioning · MySQL/SQLite · REPL `tinker`

> Voir la [roadmap détaillée](./docs/ROADMAP.md).

## Architecture en bref

La CLI `afrivel` est écrite en **Go (Cobra)** et orchestre un runtime écrit en **crates Rust**.
Elle gère le scaffolding, la génération de code et la boucle de dev, puis **délègue** à `cargo`
tout ce qui nécessite la base de données. Détails dans [`docs/ARCHITECTURE.md`](./docs/ARCHITECTURE.md).

## Contribuer

Afrivel est open source et accueille toutes les contributions : code, documentation, tests,
traductions, idées. Le design complet vit dans [`/docs`](./docs/README.md) — commence par là.

## Licence

Double licence **MIT OR Apache-2.0**, au choix — standard de l'écosystème Rust.
Voir [`LICENSE-MIT`](./LICENSE-MIT) et [`LICENSE-APACHE`](./LICENSE-APACHE).

> Toolchain : Rust **stable**, edition **2024** (MSRV 1.85+).

---

<div align="center">

*Construisons ensemble le Laravel de l'écosystème Rust.*

</div>
