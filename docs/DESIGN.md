# Afrivel — Document de Design (v0.0.1)

> Issu d'une session de brainstorming structuré. Ce document est la **source de vérité du design** avant implémentation.

---

## 1. Understanding Summary

- **Quoi** : un framework web Rust OSS « batteries-included », couche de **productivité + génération de code** au-dessus d'**Axum + Tower + Tokio**. Fonctionnalité signature : `afrivel make:module` génère un module métier autonome complet.
- **Pourquoi** : combler l'absence d'un full-stack « Laravel-like » productif en Rust, sans sacrifier performance, sécurité mémoire et concurrence.
- **Pour qui** : pont **équitable** entre devs Laravel/Rails migrants et Rustaceans cherchant du batteries-included (tension de design assumée, arbitrée cas par cas).
- **Comment** : ORM ergonomique au-dessus de **SeaORM/sqlx** ; enregistrement des modules/routes **explicite** maintenu par la CLI (pas de réflexion runtime) ; **CLI en Go + Cobra** orchestrant un runtime Rust ; auto-reload (watch/recompile/restart, pas de hot-swap).
- **Objectif** : framework OSS sérieux destiné à l'adoption → rigueur d'API, docs, tests, CI dès le départ.

## 2. Assumptions

| # | Hypothèse | Statut |
|---|-----------|--------|
| A1 | Performance : overhead négligeable vs Axum brut ; la valeur = DX, pas battre Axum. | Assumé |
| A2 | Échelle cible : apps SaaS/API typiques ; pas de benchmark extrême en v0.0.1. | Assumé |
| A3 | Sécurité : défauts sains (Argon2, validation stricte, secrets via env/config). | Assumé |
| A4 | Fiabilité : SemVer ; `0.x` = breaking changes tolérés mais documentés. | Assumé |
| A5 | Templates de codegen : Go `text/template` + `go:embed`. | Validé |
| A6 | Licence : `MIT OR Apache-2.0` (standard Rust). | **À confirmer** |
| A7 | Édition Rust stable (pas de nightly requis pour utiliser le framework). | **À confirmer** |

## 3. Non-Functional Requirements

- **Performance** : pas de coût caché significatif au-dessus d'Axum ; le codegen produit du code idiomatique compilé.
- **Sécurité** : hashing Argon2 par défaut, validation stricte des Requests, secrets hors du code, pas de `unsafe` non justifié.
- **Fiabilité** : API publique versionnée ; sortie de `make:*` **toujours compilable**.
- **Maintenance** : projet OSS — docs, tests (dont compilation réelle en CI), gouvernance des contributions.
- **Portabilité** : CLI Go = binaire statique Linux/macOS/Windows ; runtime Rust multiplateforme.

## 4. Non-Goals (v0.0.1)

OAuth2, queues/jobs/scheduler, events, API versioning, multi-BDD (MySQL/SQLite), frontend/templating serveur, admin UI, hot-swap de code, REPL `tinker`. **Namespaces réservés** mais non implémentés.

## 5. Périmètre v0.0.1

- **Core** : routing (Axum), middleware, config typée, logging, validation, DI léger.
- **ORM** : CRUD **+ relations** (requis par Auth), migrations, factories, seeders, **Postgres** d'abord.
- **CLI** : `new`, `make:module` (+ `--model`), `make:*` granulaires, `migrate*`, `db:seed`, `serve`, `dev`, `module:list`, `route:list`, `completion`.
- **Module Auth complet** : JWT, RBAC, permissions, hashing Argon2.
- **Démo** : 1 app end-to-end + auto-reload + suite de tests.

> ⚠️ Choisir « Auth complet » tire mécaniquement Core + ORM relationnel + middleware dans la v0.0.1. Jalon volontairement **conséquent**.

## 6. Risques majeurs

| Risque | Mitigation |
|--------|------------|
| Tension Laravel-familier vs idiomatique-Rust | Arbitrage cas par cas, documenté dans le Decision Log. |
| Concurrent direct **Loco.rs** (même créneau/stack) | Différenciation : approche **module-centric**, DX **bilingue** Laravel↔Rust, CLI Go riche (complétion, codegen transactionnel). **À approfondir.** |
| Codegen produisant du code non-compilable | Test d'intégration `cargo build` réel + golden files (garde-fou central). |
| Architecture bi-langage (Go ↔ Rust) | Frontière stricte : la CLI Go ne touche jamais la BDD ni ne parse du Rust ; délègue à `cargo`. |
| ORM Active-Record « magique » impossible en Rust | On bâtit sur SeaORM/sqlx + ergonomie via macros `derive`, pas de magie runtime. |

## 7. Architecture (vue d'ensemble)

```
┌─ afrivel (binaire Go/Cobra) ───────────────┐   ┌─ Runtime Rust (crates) ─────┐
│ make:* (scaffolding + codegen, go:embed)   │   │ afrivel-core   (routing,DI) │
│ new        (bootstrap projet)              │   │ afrivel-orm    (SeaORM++)   │
│ dev/watch  (recompile+restart)             │──▶│ afrivel-cli-rt (sous-cmd    │
│ manifest   (lecture/écriture registre)     │   │   migrate/serve/seed)       │
│ route:list, module:list (lit manifeste)    │   │ afrivel-macros (derive)     │
│ migrate/serve/seed  ──délègue──▶ cargo run │   │                             │
└────────────────────────────────────────────┘   └─────────────────────────────┘
```

**Frontière directrice** : la CLI Go ne touche jamais la BDD et ne parse jamais du Rust. Toute commande exigeant le runtime est déléguée à `cargo run` sur le binaire `src/bin/afrivel.rs` généré dans le projet.

Voir [`CLI.md`](./CLI.md) pour la spécification détaillée des commandes, et [`ARCHITECTURE.md`](./ARCHITECTURE.md) pour le layout et le double registre.
