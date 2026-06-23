# Afrivel — Document de Design (v0.0.1)

> Issu d'une session de brainstorming structuré. Ce document est la **source de vérité du design** avant implémentation.

---

## 1. Understanding Summary

- **Quoi** : un framework web Rust OSS « batteries-included », couche de **productivité + génération de code** au-dessus d'**Axum + Tower + Tokio**. Fonctionnalité signature : `afrivel make:module` génère un module métier **encapsulé** complet (crate dédiée, dépendances inter-modules explicites).
- **Pourquoi** : combler l'absence d'un full-stack « Laravel-like » productif en Rust, sans sacrifier performance, sécurité mémoire et concurrence.
- **Pour qui** : pont **équitable** entre devs Laravel/Rails migrants et Rustaceans cherchant du batteries-included (tension de design assumée, arbitrée cas par cas).
- **Comment** : ORM ergonomique au-dessus de **SeaORM/sqlx** ; enregistrement des modules/routes **explicite** maintenu par la CLI (pas de réflexion runtime) ; **CLI unique en Rust (clap)** — binaire global `afrivel` qui délègue le runtime au binaire d'app du projet ; auto-reload (watch/recompile/restart, pas de hot-swap).
- **Objectif** : framework OSS sérieux destiné à l'adoption → rigueur d'API, docs, tests, CI dès le départ.

## 2. Assumptions

| # | Hypothèse | Statut |
|---|-----------|--------|
| A1 | Performance : overhead négligeable vs Axum brut ; la valeur = DX, pas battre Axum. | Assumé |
| A2 | Échelle cible : apps SaaS/API typiques ; pas de benchmark extrême en v0.0.1. | Assumé |
| A3 | Sécurité : défauts sains (Argon2, validation stricte, secrets via env/config). | Assumé |
| A4 | Fiabilité : SemVer ; `0.x` = breaking changes tolérés mais documentés. | Assumé |
| A5 | Codegen : templates `minijinja` embarqués (`rust-embed`), CLI Rust unique (clap). | Validé (DR-025) |
| A6 | Licence : `MIT OR Apache-2.0` (dual). | Validé (DR-015) |
| A7 | Toolchain : Rust stable, edition 2024, MSRV 1.85+. | Validé (DR-016) |
| A8 | Différenciation : module-centric + Clean Architecture (vs Loco.rs). | Validé (DR-017) |

## 3. Non-Functional Requirements

- **Performance** : pas de coût caché significatif au-dessus d'Axum ; le codegen produit du code idiomatique compilé.
- **Sécurité** : hashing Argon2 par défaut, validation stricte des Requests, secrets hors du code, pas de `unsafe` non justifié.
- **Fiabilité** : API publique versionnée ; sortie de `make:*` **toujours compilable** ; gestion d'erreurs unifiée (`afrivel::Error` → réponses HTTP normalisées).
- **Maintenabilité** : frontières de crate par module (encapsulation réelle) + règle de dépendance Clean Architecture imposée par défaut.
- **Maintenance** : projet OSS — docs, tests (dont compilation réelle en CI), gouvernance des contributions.
- **Portabilité** : CLI Rust = binaire multiplateforme (Linux/macOS/Windows), `cargo install` + binaires pré-compilés ; toute la chaîne mono-langage.

## 4. Non-Goals (v0.0.1)

OAuth2, queues/jobs/scheduler, events, API versioning, multi-BDD (MySQL/SQLite), frontend/templating serveur, admin UI, hot-swap de code, REPL `tinker`. **Namespaces réservés** mais non implémentés.

## 5. Périmètre v0.0.1

- **Core** : routing (Axum), middleware, **type d'erreur unifié → IntoResponse**, config typée (serde + figment, TOML+env), logging structuré (`tracing`), validation, DI compile-time (trait objects + Axum State/Extension).
- **Architecture** : projet = **Cargo workspace** ; chaque module = **crate** appliquant la Clean Architecture (couches `http → services → contracts ← repositories`, domaine sans dépendance infra). Voir [ARCHITECTURE.md](./ARCHITECTURE.md).
- **ORM** : CRUD **+ relations** (requis par Auth), migrations **ordonnées par timestamp**, factories, seeders, **Postgres** d'abord.
- **CLI** : `new`, `make:module` (+ `--model`), `make:*` granulaires, `migrate*`, `db:seed`, `serve`, `dev`, `module:list`, `route:list`, `completion`.
- **Module Auth complet** : JWT, RBAC, permissions, hashing Argon2.
- **Démo** : 1 app end-to-end + auto-reload + suite de tests.

> ⚠️ Choisir « Auth complet » tire mécaniquement Core + ORM relationnel + middleware dans la v0.0.1. Jalon volontairement **conséquent**.

## 6. Risques majeurs

| Risque | Mitigation |
|--------|------------|
| Tension Laravel-familier vs idiomatique-Rust | Arbitrage cas par cas, documenté dans le Decision Log. |
| Concurrent direct **Loco.rs** (même créneau/stack) | Différenciation tranchée (DR-017) : **module-centric + Clean Architecture par défaut** (couches Services/Repositories/Interfaces), cible apps maintenables en équipe — vs l'approche Rails-fine de Loco. |
| Codegen produisant du code non-compilable | Test d'intégration `cargo build` réel + golden files (garde-fou central). |
| Couplage CLI globale ↔ crates du projet | Mono-langage Rust : contrat des sous-commandes runtime partagé via la crate `afrivel-cli-rt` (vérifié à la compilation) ; version signalée via `afrivel_version`. |
| ORM Active-Record « magique » impossible en Rust | On bâtit sur SeaORM/sqlx + ergonomie via macros `derive`, pas de magie runtime. |

## 7. Architecture (vue d'ensemble)

```
┌─ afrivel  (crate afrivel-cli, clap) ───────┐   ┌─ app du projet (lie les modules) ─┐
│ GLOBAL — cargo install afrivel             │   │ serve / migrate* / db:seed /      │
│ new · make:* (codegen minijinja) · dev     │──▶│   route:list                      │
│ module:list (lit Afrivel.toml)             │   │ via afrivel-cli-rt (clap partagé) │
│ migrate/seed/serve/route:list ─délègue─▶   │   │ deps: afrivel-core/orm/macros     │
│        cargo run -p app -- <sous-cmd>      │   │                                   │
└────────────────────────────────────────────┘   └───────────────────────────────────┘
            (toute la chaîne en Rust — mono-langage)
```

**Frontière directrice** : le binaire global `afrivel` ne lie pas les modules d'un projet → il **délègue** les commandes runtime (BDD ou introspection du routeur, dont `route:list`) au binaire `app` du projet via `cargo run -p app -- <sous-cmd>`. Contrat partagé via `afrivel-cli-rt` (Rust↔Rust, vérifié à la compilation).

Voir [`CLI.md`](./CLI.md) pour la spécification détaillée des commandes, et [`ARCHITECTURE.md`](./ARCHITECTURE.md) pour le layout et le double registre.
