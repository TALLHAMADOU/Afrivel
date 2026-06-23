# Afrivel — Architecture

## Frontière Go ↔ Rust

```
┌─ afrivel (binaire Go/Cobra) ───────────────┐   ┌─ Runtime Rust (crates framework) ─┐
│ make:* (scaffolding + codegen, go:embed)   │   │ afrivel-core   (routing,erreurs,  │
│ new        (bootstrap projet)              │   │   config, DI, validation, tracing)│
│ dev/watch  (recompile+restart)             │──▶│ afrivel-orm    (SeaORM++)         │
│ manifest   (lecture/écriture Afrivel.toml) │   │ afrivel-macros (derive)           │
│ module:list (lit manifeste)                │   │ afrivel-cli-rt (sous-cmd runtime  │
│ migrate/seed/serve/route:list ─délègue─▶   │   │   migrate/seed/serve/route:list,  │
│        cargo run -p app -- <sous-cmd>      │   │   via clap)                       │
└────────────────────────────────────────────┘   └───────────────────────────────────┘
```

**Invariants :**
1. La CLI Go ne se connecte **jamais** à la BDD.
2. La CLI Go ne **parse jamais** du code Rust. Elle lit/écrit uniquement `Afrivel.toml` et des fichiers `Cargo.toml` (TOML).
3. Toute commande exigeant le runtime (BDD **ou** introspection du routeur) est déléguée à `cargo run -p app -- <sous-commande>`.
4. La sortie de tout `make:*` **compile** (`cargo build` vert).

## Catégories de commandes

| Catégorie | Exécution | Exemples |
|-----------|-----------|----------|
| Pures Go (hors-ligne) | binaire Go seul | `new`, `make:*`, `module:list`, `completion` |
| Déléguées | `cargo run -p app` → runtime Rust | `migrate*`, `db:seed`, `serve`, **`route:list`** |
| Hybrides | Go surveille + délègue | `dev` / `watch` |

> **`route:list` est délégué** (correction architecturale) : les routes sont définies en Rust, pas dans le manifeste. Seul le runtime, qui monte le routeur Axum, connaît la table de routes exacte. La lister sans parser du Rust est impossible → délégation obligatoire (invariant n°2). `module:list`, lui, reste pur Go (les modules **sont** dans `Afrivel.toml`).

Hors d'un projet Afrivel (`Afrivel.toml` absent) : seules `new` et l'aide fonctionnent.

## Layout d'un projet généré — **Cargo workspace** (`afrivel new myapp`)

```
myapp/
├── Cargo.toml            # [workspace] members = ["app", "modules/*"]
├── Afrivel.toml          # marqueur projet + manifeste outillage (modules, deps, db)
├── .env / .env.example   # DATABASE_URL, APP_KEY, secrets
├── app/                  # crate binaire = l'application
│   ├── Cargo.toml        # deps: afrivel-core, afrivel-orm, + chaque module (path dep)
│   └── src/
│       ├── main.rs       # point d'entrée : dispatch sous-commandes (afrivel-cli-rt)
│       │                 #   sans arg/serve → serveur ; migrate/seed/route:list → tâches
│       ├── registry.rs   # REGISTRE COMPILATION : register_all() câble chaque module
│       └── migrator.rs   # agrège les migrations de tous les modules (ordre = timestamp)
├── modules/
│   ├── auth/             # crate autonome
│   │   ├── Cargo.toml    # deps: afrivel-core/orm (+ autres modules si nécessaire)
│   │   └── src/…         # voir « Anatomie d'un module »
│   └── payment/          # crate ; dépend de `auth` (path dep) pour le contrat User
├── config/               # *.toml chargés en structs typées (serde + figment)
├── database/
│   ├── migrations/       # migrations globales/initiales (non liées à un module)
│   └── seeders/
├── storage/              # logs, cache, uploads (ignoré par le watcher)
└── tests/                # tests d'intégration cross-module (crate de test)
```

**Pourquoi un workspace** : chaque module est une **vraie frontière de compilation** (encapsulation réelle, builds incrémentaux isolés, dépendances inter-modules explicites). C'est ce qui rend le « module-centric + Clean Architecture » concret plutôt que cosmétique. Voir [DECISIONS.md DR-018].

## Anatomie d'un module (`modules/auth/`)

Crate Rust appliquant la **Clean Architecture** (dépendances vers l'intérieur) :

```
auth/
├── Cargo.toml
└── src/
    ├── lib.rs            # expose module() -> Module ; pub use des contrats
    ├── module.rs         # Module trait : enregistre routes + câble repos→services (DI)
    ├── contracts/        # INTERFACES (traits) : surface publique consommée par d'autres modules
    ├── domain/
    │   ├── models/       # entités métier — NE dépendent d'aucune couche infra
    │   └── services/     # logique métier — dépendent des contracts (traits), pas des repos
    ├── http/
    │   ├── controllers/  # adaptateurs HTTP (handlers Axum) — dépendent des services
    │   ├── requests/     # validation/désérialisation des entrées
    │   ├── resources/    # sérialisation des sorties (DTO de réponse)
    │   └── routes.rs     # routes du module
    ├── infra/
    │   └── repositories/ # IMPLÉMENTENT les contracts via afrivel-orm/SeaORM
    ├── migrations/       # migrations possédées par le module (préfixe timestamp)
    └── tests/
```

### Règle de dépendance (Clean Architecture)

```
http (controllers/requests/resources)
        │ dépend de
        ▼
   services ───dépend de──▶ contracts (traits)
        │                        ▲
        │ dépend de              │ implémentent
        ▼                        │
     models (domaine)      repositories (infra)
   (ne dépend de rien)
```

- **Inversion de dépendance** : les `services` dépendent de **traits** (`contracts`), jamais des `repositories` concrets. Le câblage trait→impl se fait à l'enregistrement du module (`module.rs`).
- Le **domaine** (`models`) ne dépend d'aucune couche d'infrastructure ni du web.
- Les modules ne se voient **que via `contracts`** (jamais l'`infra` ou les `models` internes d'un autre module).

## Dépendances inter-modules

« Autonome » = **encapsulé**, pas **isolé**. Un module qui en consomme un autre déclare une **dépendance de crate explicite** + n'utilise que ses `contracts` :

```toml
# modules/payment/Cargo.toml
[dependencies]
auth = { path = "../auth" }   # payment utilise auth::contracts::UserRef
```

La CLI enregistre ces dépendances dans `Afrivel.toml` (`deps = ["auth"]`) et les ajoute au `Cargo.toml` du module. `make:module Payment --depends auth` câble le tout.

## Migrations & ordonnancement

Problème : Auth crée `users`, Payment a une FK `user_id` → l'ordre compte.

- Chaque migration porte un **préfixe timestamp** type Laravel : `2026_06_23_120000_create_users`.
- `app/src/migrator.rs` **agrège** les migrations de tous les modules + `database/migrations/` et les **trie par timestamp** → ordre déterministe, indépendant des modules.
- La CLI attribue le timestamp à la génération → un module généré plus tard migre après. Pour une dépendance explicite, `make:migration` accepte le décalage si besoin.
- Source de vérité de l'ordre = les timestamps (pas le manifeste). SeaORM `Migrator` reçoit la liste triée.

## DI (Dependency Injection)

Pas de conteneur runtime (non idiomatique en Rust). Le mécanisme :
- Les `repositories` sont injectés comme **trait objects** (`Arc<dyn UserRepository>`).
- Câblage à l'enregistrement du module (`module.rs`) ; partagés aux handlers via **Axum `State`/`Extension`**.
- C'est exactement le couple `contracts` (trait) + `repositories` (impl) qui réalise l'inversion de dépendance — la « DI » d'Afrivel est **compile-time et explicite**.

## Gestion d'erreurs du framework

- `afrivel-core` fournit `afrivel::Error` (enum : `Validation`, `NotFound`, `Unauthorized`, `Database`, `Internal`…) + `afrivel::Result<T>`.
- `Error` implémente **`IntoResponse`** : mapping erreur → statut HTTP + corps JSON normalisé.
- Les couches métier retournent `afrivel::Result` ; les controllers propagent via `?`. Conversion depuis `sea_orm::DbErr`, erreurs de validation, etc., via `From`.

## Double registre

| Registre | Fichier | Vérité de | Maintenu par |
|----------|---------|-----------|--------------|
| Outillage | `Afrivel.toml` | la CLI (introspection rapide, hors-ligne) | CLI Go |
| Compilation | `app/src/registry.rs` + path-deps dans `app/Cargo.toml` & `modules/*/Cargo.toml` | le compilateur Rust | CLI Go (édition) + rustc |

**Résolution de conflit** : le code Rust (qui compile) fait foi ; `Afrivel.toml` est régénérable (`afrivel doctor`, post-v0.0.1).

### Exemple `Afrivel.toml`
```toml
[project]
name = "myapp"
afrivel_version = "0.0.1"

[database]
default = "postgres"

[modules.auth]
path = "modules/auth"
model = "User"
deps = []

[modules.payment]
path = "modules/payment"
deps = ["auth"]
```

## Crates runtime (framework, dépendances externes)

| Crate | Responsabilité |
|-------|----------------|
| `afrivel-core` | Routing (Axum/Tower), middleware, **type d'erreur + IntoResponse**, config typée (serde + figment), validation, logging (`tracing`), DI (State/Extension). |
| `afrivel-orm` | Couche ergonomique sur SeaORM/sqlx : relations, scopes, factories, seeders, agrégation de migrations. |
| `afrivel-macros` | `#[derive(Model)]`, dérivations Request/Resource. |
| `afrivel-cli-rt` | Jeu de sous-commandes runtime (migrate/seed/serve/route:list) exposé via clap, monté dans `app/src/main.rs`. **Contrat couplé à la CLI Go** — versionné via `afrivel_version` (DR-022). |

> Note : les crates `afrivel-*` sont des **dépendances** (registre/git), pas des membres du workspace de l'app. Les membres du workspace sont `app` + `modules/*`.
