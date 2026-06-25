# Afrivel — Roadmap

## v0.0.1 — Le jalon « preuve de valeur »

Objectif : un développeur peut `new` un projet, générer un module Auth complet par CLI, migrer, et lancer une API qui tourne — end-to-end.

### Core (`afrivel-core`) — ✅ M1
- [x] Routing au-dessus d'Axum (`Application` + trait `Module`)
- [x] Middleware (pipeline Tower : `TraceLayer`, couches injectables)
- [x] **Type d'erreur unifié `afrivel::Error` + `IntoResponse`**
- [x] Configuration typée (serde + figment, `config/` + `.env`)
- [x] Logging structuré (`tracing` + `tracing-subscriber`)
- [x] Validation (Requests : trait `Validate` + extracteur `ValidatedJson`)
- [x] DI compile-time (`Application::provide` → `Extension<Arc<dyn _>>`)
- [ ] Contrat `afrivel-cli-rt` (sous-commandes runtime via clap) + garde de version — *M3*

### Architecture du projet généré
- [ ] Scaffolding **Cargo workspace** (`app/` + `modules/*`)
- [ ] Squelette de module en Clean Architecture (`http/services/contracts/infra/domain`)
- [ ] Dépendances inter-modules explicites (`--depends`, path-deps, `contracts`)
- [ ] Agrégation + tri des migrations par timestamp (`app/src/migrator.rs`)

### ORM (`afrivel-orm`, sur SeaORM/sqlx) — ✅ M2
- [x] CRUD (`repository::{create,update,find,find_or_fail,all,delete}` + trait `Model`)
- [x] Relations (1-1, 1-N, N-N) — pivot N-N migré + testé sur Postgres
- [x] Migrations (agrégation + tri par timestamp `migrator::sorted`, DR-021)
- [x] Factories (trait `Factory` : `definition`/`create`/`create_many`)
- [x] Seeders (trait `Seeder` dyn-compatible + `run_seeders`)
- [x] Postgres (driver unique en v0.0.1 ; test d'intégration en CI)

### Codegen & macros — ✅ M2
- [x] `afrivel-codegen` : parser DSL `--model`, mapping de types, naming (18 tests)
- [x] `afrivel-macros` : `#[derive(Model)]` (lien entité + CRUD ergonomique)
- [ ] Dérivations `Request`/`Resource` — *M3/M4 (couche HTTP)*

### CLI (`afrivel-cli`, Rust + clap)
- [ ] `new`
- [ ] `make:module` (+ `--model` DSL, codegen transactionnel, templates `minijinja`)
- [ ] `make:*` granulaires
- [ ] `migrate*`, `db:seed` (délégués au binaire `app`)
- [ ] `serve`, `dev` (watch via `notify` / recompile / restart)
- [ ] `module:list` (hors-ligne), `route:list` (délégué)
- [ ] `completion` (`clap_complete`)
- [ ] `cargo install afrivel` + binaires pré-compilés
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
