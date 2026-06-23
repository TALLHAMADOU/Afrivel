# Afrivel — Architecture

## Frontière Go ↔ Rust

```
┌─ afrivel (binaire Go/Cobra) ───────────────┐   ┌─ Runtime Rust (crates) ─────┐
│ make:* (scaffolding + codegen, go:embed)   │   │ afrivel-core   (routing,DI) │
│ new        (bootstrap projet)              │   │ afrivel-orm    (SeaORM++)   │
│ dev/watch  (recompile+restart)             │──▶│ afrivel-cli-rt (migrate/    │
│ manifest   (lecture/écriture registre)     │   │   serve/seed via clap)      │
│ route:list, module:list (lit manifeste)    │   │ afrivel-macros (derive)     │
│ migrate/serve/seed  ──délègue──▶ cargo run │   │                             │
└────────────────────────────────────────────┘   └─────────────────────────────┘
```

**Invariants :**
1. La CLI Go ne se connecte **jamais** à la BDD.
2. La CLI Go ne **parse jamais** du code Rust (elle lit le manifeste TOML).
3. Toute commande runtime est déléguée à `cargo run --bin afrivel -- <sous-commande>`.
4. La sortie de tout `make:*` **compile**.

## Catégories de commandes

| Catégorie | Exécution | Exemples |
|-----------|-----------|----------|
| Pures Go (hors-ligne) | binaire Go seul | `new`, `make:*`, `module:list`, `route:list`, `completion` |
| Déléguées | `cargo run` → runtime Rust | `migrate*`, `db:seed`, `serve` |
| Hybrides | Go surveille + délègue | `dev` / `watch` |

Hors d'un projet Afrivel (`Afrivel.toml` absent) : seules `new` et l'aide fonctionnent.

## Layout d'un projet généré (`afrivel new myapp`)

```
myapp/
├── Afrivel.toml          # marqueur projet + manifeste (modules, config CLI)
├── Cargo.toml            # deps: afrivel-core, afrivel-orm, sea-orm…
├── .env / .env.example   # DATABASE_URL, APP_KEY, secrets
├── src/
│   ├── main.rs           # bootstrap app → afrivel-core
│   └── bin/afrivel.rs    # binaire délégué (migrate/serve/seed via afrivel-cli-rt)
├── modules/
│   └── mod.rs            # registre Rust: `pub mod auth;` + `register(auth::module())`
├── config/               # config typée
├── database/
│   ├── migrations/       # registre des migrations
│   └── seeders/
├── storage/              # logs, cache, uploads
├── routes/               # routes globales (hors modules)
└── tests/                # tests d'intégration cross-module
```

## Anatomie d'un module (`modules/Auth/`)

Chaque module est **autonome** :

```
Auth/
├── mod.rs                # Module trait : enregistre routes/services du module
├── models/      requests/      controllers/   services/
├── interfaces/  repositories/  resources/
├── migrations/  routes/        tests/
```

## Double registre

| Registre | Fichier | Vérité de | Maintenu par |
|----------|---------|-----------|--------------|
| Outillage | `Afrivel.toml` | la CLI (introspection rapide, hors-ligne) | CLI Go |
| Compilation | `modules/mod.rs` | le compilateur Rust | CLI Go (édition) + rustc |

**Résolution de conflit** : le code Rust (`mod.rs`) fait foi ; `Afrivel.toml` est régénérable (`afrivel doctor`, post-v0.0.1).

### Exemple `Afrivel.toml`
```toml
[project]
name = "myapp"
afrivel_version = "0.0.1"

[database]
default = "postgres"

[modules]
auth    = { path = "modules/Auth",    model = "User" }
payment = { path = "modules/Payment" }
```

## Crates runtime (Rust)

| Crate | Responsabilité |
|-------|----------------|
| `afrivel-core` | Routing (Axum), middleware, config, logging, validation, DI léger. |
| `afrivel-orm` | Couche ergonomique sur SeaORM/sqlx : relations, scopes, factories, seeders. |
| `afrivel-macros` | `#[derive(Model)]`, dérivations de Request/Resource. |
| `afrivel-cli-rt` | Sous-commandes runtime (migrate/serve/seed) exposées via clap, montées dans `src/bin/afrivel.rs`. |
