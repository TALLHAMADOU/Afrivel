# Afrivel — Roadmap

## v0.0.1 — Le jalon « preuve de valeur »

Objectif : un développeur peut `new` un projet, générer un module Auth complet par CLI, migrer, et lancer une API qui tourne — end-to-end.

### Core (`afrivel-core`)
- [ ] Routing au-dessus d'Axum
- [ ] Middleware (pipeline Tower)
- [ ] Configuration typée (`config/`, `.env`)
- [ ] Logging
- [ ] Validation (Requests)
- [ ] DI léger

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

## Décisions à trancher avant/pendant v0.0.1

1. **Licence** : `MIT OR Apache-2.0` (proposé, standard Rust) — à confirmer.
2. **Édition Rust** : stable requis pour les utilisateurs — à confirmer (impacte les macros).
3. **Différenciation vs Loco.rs** : angle exact (module-centric + DX bilingue + CLI Go riche) — à approfondir avant communication publique.
