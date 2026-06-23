# Afrivel — Roadmap

## v0.0.1 — Le jalon « preuve de valeur »

Objectif : un développeur peut `new` un projet, générer un module Auth complet par CLI, migrer, et lancer une API qui tourne — end-to-end.

### Core (`afrivel-core`)
- [ ] Routing au-dessus d'Axum
- [ ] Middleware (pipeline Tower)
- [ ] **Type d'erreur unifié `afrivel::Error` + `IntoResponse`**
- [ ] Configuration typée (serde + figment, `config/` + `.env`)
- [ ] Logging structuré (`tracing` + `tracing-subscriber`)
- [ ] Validation (Requests)
- [ ] DI compile-time (trait objects + Axum State/Extension)
- [ ] Contrat `afrivel-cli-rt` (sous-commandes runtime via clap) + garde de version

### Architecture du projet généré
- [ ] Scaffolding **Cargo workspace** (`app/` + `modules/*`)
- [ ] Squelette de module en Clean Architecture (`http/services/contracts/infra/domain`)
- [ ] Dépendances inter-modules explicites (`--depends`, path-deps, `contracts`)
- [ ] Agrégation + tri des migrations par timestamp (`app/src/migrator.rs`)

### ORM (`afrivel-orm`, sur SeaORM/sqlx)
- [ ] CRUD
- [ ] Relations (1-1, 1-N, N-N) — requis par Auth
- [ ] Migrations (registre + runner délégué)
- [ ] Factories
- [ ] Seeders
- [ ] Postgres (driver unique en v0.0.1)

### CLI (`afrivel`, Go/Cobra)
- [ ] `new`
- [ ] `make:module` (+ `--model` DSL, codegen transactionnel)
- [ ] `make:*` granulaires
- [ ] `migrate*`, `db:seed` (délégués)
- [ ] `serve`, `dev` (watch/recompile/restart)
- [ ] `module:list`, `route:list`
- [ ] `completion`
- [ ] Tests : golden files + compilation réelle

### Module Auth (généré)
- [ ] JWT
- [ ] RBAC (users ↔ roles ↔ permissions)
- [ ] Permissions
- [ ] Hashing Argon2

### Qualité
- [ ] App démo end-to-end
- [ ] CI matrix (OS × Rust stable)
- [ ] Docs utilisateur (README racine) + docs dev (`/docs`)

---

## Post-v0.0.1 (namespaces réservés, non implémentés)

- **Auth** : OAuth2
- **Background** : Jobs, Queues, Events, Scheduler (`make:job`, `queue:work`, `schedule:run`)
- **API** : Resource Transformers avancés, API Versioning
- **BDD** : MySQL, SQLite
- **DX** : `tinker` (REPL), `afrivel doctor` (détection de dérives manifeste↔code), reload assets à chaud étendu
- **Core** : `make:command` (commandes CLI custom)

---

## Décisions tranchées

1. **Licence** : `MIT OR Apache-2.0` (dual). ✅
2. **Toolchain** : Rust stable, **edition 2024**, MSRV 1.85+. ✅
3. **Différenciation vs Loco.rs** : Module-centric + Clean Architecture (couches Services/Repositories/Interfaces imposées par défaut ; cible apps maintenables en équipe). ✅
