<div align="center">

# Afrivel

**L'expérience Laravel, propulsée par Rust.**

Scaffolder un backend web complet — modules, ORM, migrations, auth — en une commande,
avec la performance et la sécurité mémoire de Rust.

[![CI](https://github.com/TALLHAMADOU/Afrivel/actions/workflows/ci.yml/badge.svg)](https://github.com/TALLHAMADOU/Afrivel/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#licence)
[![Rust 2024](https://img.shields.io/badge/rust-stable%20(MSRV%201.85)-orange.svg)](#prérequis)

[Quickstart](./docs/QUICKSTART.md) · [Documentation](./docs/README.md) · [Référence CLI](./docs/CLI.md) · [Roadmap](./docs/ROADMAP.md)

</div>

---

> **Statut : `v0.0.1` — premier socle fonctionnel.**
> Le cœur, la CLI, l'ORM et un module Auth de référence fonctionnent et sont testés de bout en
> bout (voir [Ce qui marche aujourd'hui](#ce-qui-marche-aujourdhui)). C'est une version **early** :
> l'API publique peut encore évoluer, et plusieurs domaines (jobs, events, scheduler) sont des
> namespaces **réservés mais non implémentés**. Pas encore recommandé en production.

## Pourquoi Afrivel

Démarrer un backend web en Rust, c'est aujourd'hui assembler à la main le routing, la base de
données, les migrations, l'authentification et la structure des dossiers — avant même d'écrire la
première règle métier. Afrivel supprime ce travail répétitif sans masquer Rust : c'est une couche
de **productivité au-dessus d'Axum, Tower, Tokio et SeaORM**, pas un nouveau serveur HTTP.

```bash
afrivel new mon-app
cd mon-app
afrivel make:module blog --model "title:string body:text published:bool"
```

En une commande, `make:module` génère une crate de module complète — modèle, requests validés,
controllers, services, contrats (traits), repositories, resources, migrations, routes et tests —
le tout en **Clean Architecture** et **garanti compilable**.

## Ce qui marche aujourd'hui

Tout est vérifié en CI (`fmt` · `clippy` · tests sur Linux/macOS/Windows + intégration Postgres).

| Domaine | État | Détail |
|---------|:----:|--------|
| **CLI** (`afrivel`) | ✅ | `new`, `make:module`, `dev` (auto-reload), délégation runtime (`serve`, `migrate*`, `route:list`). |
| **Cœur** (`afrivel-core`) | ✅ | `Application` (registre de modules + DI par `provide`), trait `Module`, `Error → IntoResponse`, config (figment), validation, logs `tracing`. |
| **ORM** (`afrivel-orm`) | ✅ | SeaORM ergonomique, migrations ordonnées par timestamp (déterministe), mapping `DbErr → Error`. |
| **Auth** (`afrivel::auth`) | ✅ | Hachage Argon2id, JWT HS256, extracteur `AuthUser`, RBAC typé `Authorized<G>`. |
| **Garde-fou codegen** | ✅ | Test e2e réel `new → make:module → cargo build` : le code généré **compile**. |
| Jobs / Queues / Events / Scheduler | ⏳ | Namespaces réservés, non implémentés ([roadmap](./docs/ROADMAP.md)). |
| OAuth2 · MySQL/SQLite · `tinker` (REPL) | ⏳ | Prévus post-`v0.0.1`. |

### Preuve : le flux d'authentification, vert de bout en bout

L'app de référence [`examples/demo`](./examples/demo) tourne en CI contre un vrai Postgres
(migrations + flux HTTP réel) :

```bash
# inscription → 201
curl -X POST localhost:3000/auth/register \
  -H 'content-type: application/json' \
  -d '{"email":"alice@example.com","password":"supersecret"}'

# connexion → 200 + jeton JWT
curl -X POST localhost:3000/auth/login \
  -H 'content-type: application/json' \
  -d '{"email":"alice@example.com","password":"supersecret"}'

# route protégée → 200 (401 sans jeton, 403 sans le rôle requis)
curl localhost:3000/auth/me -H "authorization: Bearer <token>"
```

Détails du module : [`docs/modules/auth.md`](./docs/modules/auth.md).

## Démarrer

```bash
cargo install afrivel        # binaire global `afrivel`
afrivel new mon-app          # crée le workspace (app, Afrivel.toml, migrator, registry)
cd mon-app
afrivel make:module blog --model "title:string body:text published:bool"

export DATABASE_URL="postgres://user:pass@localhost:5432/mon_app"
afrivel migrate              # applique les migrations
afrivel dev                  # serveur + auto-reload (watch · recompile · restart)
```

Guide complet : **[Quickstart](./docs/QUICKSTART.md)** · liste des commandes : **[Référence CLI](./docs/CLI.md)**.

### Prérequis

Rust **stable**, edition **2024** (MSRV **1.85+**). Postgres pour les fonctions base de données.

## En quoi c'est différent

- **vs Axum brut** — Afrivel garde Axum dessous mais ajoute les conventions, le scaffolding et le
  câblage qu'on réécrit sinon à chaque projet.
- **vs Loco.rs** — Afrivel est **module-centric** : chaque module est une crate isolée en Clean
  Architecture (`http → services → contracts ← repositories`), pas un dossier de plus dans un
  monolithe.
- **Aucune réflexion runtime** — l'enregistrement est **explicite** (`Afrivel.toml` + `registry.rs`).
  Le code Rust fait foi ; ce qui compile est ce qui tourne.

### Pensé pour le développement assisté par agents

La structure prévisible, le codegen déterministe et la garantie « toujours compilable » donnent un
terrain fiable aux agents (**Claude Code**, **Cursor**, **Copilot**) : chaque module suit la même
architecture en couches, et chaque `make:*` produit du code qui compile.

## Architecture du dépôt

| Crate | Rôle |
|-------|------|
| [`afrivel`](./crates/afrivel) | Façade : ré-exporte le cœur, l'ORM, les macros, `axum`, `tokio`. |
| [`afrivel-core`](./crates/afrivel-core) | `Application`, `Module`, DI, erreurs, config, validation, `afrivel::auth`. |
| [`afrivel-orm`](./crates/afrivel-orm) · [`afrivel-macros`](./crates/afrivel-macros) | Persistance SeaORM + derive `Model`. |
| [`afrivel-cli`](./crates/afrivel-cli) · [`afrivel-cli-rt`](./crates/afrivel-cli-rt) | Binaire `afrivel` + contrat runtime partagé CLI ↔ app. |
| [`examples/demo`](./examples/demo) | App de référence (module Auth), testée en CI. |

## Documentation

- [Quickstart](./docs/QUICKSTART.md) — de l'installation au premier endpoint protégé.
- [Architecture](./docs/ARCHITECTURE.md) — workspace, anatomie d'un module, double registre.
- [Référence CLI](./docs/CLI.md) — toutes les commandes et le codegen `make:module`.
- [Module Auth](./docs/modules/auth.md) — primitives, couches, routes, RBAC.
- [Roadmap](./docs/ROADMAP.md) · [Décisions d'architecture](./docs/DECISIONS.md) · [Changelog](./CHANGELOG.md).

## Contribuer

Les contributions sont les bienvenues — code, docs, tests, idées. Commence par le
[Quickstart](./docs/QUICKSTART.md) et l'[architecture](./docs/ARCHITECTURE.md), puis les
[décisions](./docs/DECISIONS.md). Merci de lire le
[Code de conduite](./CODE_OF_CONDUCT.md) avant de participer.

## Sécurité

Tu as trouvé une vulnérabilité ? Signale-la en privé via un **avis de sécurité GitHub**
(`Security → Report a vulnerability`) plutôt qu'en issue publique. Voir [`SECURITY.md`](./SECURITY.md).

## Licence

Double licence **MIT OR Apache-2.0**, au choix — le standard de l'écosystème Rust.
Voir [`LICENSE-MIT`](./LICENSE-MIT) et [`LICENSE-APACHE`](./LICENSE-APACHE).
