# Afrivel — Plan d'implémentation v0.0.1

> Plan séquencé pour passer du design au premier jalon utilisable. Découpé en jalons
> (M0→M5) avec **tâches**, **critères de sortie testables** et **risques**. Aligné sur
> [DESIGN.md](./DESIGN.md), [ARCHITECTURE.md](./ARCHITECTURE.md), [DECISIONS.md](./DECISIONS.md).

## Définition de « fini » (v0.0.1)

Un développeur peut, depuis zéro :

```bash
cargo install afrivel        # (ou binaire pré-compilé)
afrivel new blog
cd blog
afrivel make:module Auth --model User:name:string,email:string:unique,password:string
afrivel migrate
afrivel serve                # API Auth fonctionnelle (register/login JWT, RBAC)
```

…et **tout compile et tourne**, avec une suite de tests verte en CI.

---

## Repo de développement du framework (ce dépôt)

Devient un **Cargo workspace** regroupant les crates `afrivel-*` + l'app démo :

```
Afrivel/
├── Cargo.toml                # [workspace] members = crates/* + examples/*
├── crates/
│   ├── afrivel/              # FAÇADE : re-export core/orm/macros (ce dont l'app dépend)
│   ├── afrivel-codegen/      # DSL --model, mapping de types, naming, ModuleSpec  (zéro dep runtime)
│   ├── afrivel-macros/       # proc-macro : #[derive(Model)], Request/Resource
│   ├── afrivel-core/         # routing, Error→IntoResponse, config, tracing, validation, DI
│   ├── afrivel-orm/          # SeaORM++ : relations, scopes, factories, seeders, migrator
│   ├── afrivel-cli-rt/       # sous-commandes runtime (clap) — PARTAGÉ CLI ↔ app
│   └── afrivel-cli/          # binaire `afrivel` : new, make:*, dev (minijinja, notify)
├── templates/                # *.jinja embarqués (rust-embed) — codegen
├── examples/
│   └── demo/                 # app générée (Auth) pour les tests end-to-end
└── docs/ …
```

### Graphe de dépendances (= ordre de construction)

```
afrivel-codegen ─┬─▶ afrivel-macros ─┐
                 │                    ├─▶ afrivel (façade) ─▶ examples/demo
afrivel-core ────┼─▶ afrivel-orm ─────┘                          ▲
                 │                                                │
                 └─▶ afrivel-cli-rt ─▶ afrivel-cli ──délègue──────┘
```

---

## M0 — Fondations du workspace

**But** : un workspace qui compile à vide + CI + hygiène.

- [ ] `Cargo.toml` workspace ; squelettes des 7 crates (`cargo new` lib/bin) qui compilent vides.
- [ ] MSRV pin (`rust-version = "1.85"`, edition 2024) ; `rust-toolchain.toml`.
- [ ] `rustfmt.toml`, `clippy` en `-D warnings`.
- [ ] CI GitHub Actions : matrix Linux/macOS/Windows × stable → `fmt` + `clippy` + `test`.
- [ ] Service Postgres dans la CI (pour M2+).

**Sortie** : `cargo build --workspace` et `cargo clippy` verts en CI sur les 3 OS.
**Risque** : faible.

---

## M1 — `afrivel-core` (le noyau HTTP)

**But** : monter une app Axum minimale via l'abstraction Afrivel.

- [ ] `afrivel::Error` (enum : `Validation`, `NotFound`, `Unauthorized`, `Forbidden`, `Database`, `Internal`) + `Result<T>` ; `impl IntoResponse` (statut + JSON normalisé) ; `From<…>`.
- [ ] `Application`/`App` : registre de routes + state partagé ; trait `Module { fn register(&self, app); }`.
- [ ] Config typée : `serde` + `figment` (TOML `config/` + surcharges env/`.env`).
- [ ] Logging : `tracing` + `tracing-subscriber` (init + middleware de trace HTTP).
- [ ] Validation : trait `Request`/`Validate` (sur `validator` ou maison) + rejet → `Error::Validation`.
- [ ] DI : helpers `State`/`Extension` pour injecter `Arc<dyn Trait>`.

**Sortie** : test d'intégration — une route `GET /health` renvoie 200 ; une erreur métier renvoie le JSON normalisé attendu.
**Risque** : moyen (ergonomie de l'API `Module` à itérer).

---

## M2 — `afrivel-orm` + `afrivel-macros` (persistance)

**But** : définir un modèle, migrer, faire du CRUD + relations.

- [ ] `afrivel-codegen` : parser du DSL `--model` (types, modificateurs `unique/nullable/default/fk`), mapping type→Rust/SQL, helpers naming (`snake_case`, pluriel, `table_name`). **Testé unitairement, source unique** (réutilisé par macros + CLI).
- [ ] `afrivel-macros` : `#[derive(Model)]` (mapping SeaORM `Entity`), dérivations `Request`/`Resource`.
- [ ] `afrivel-orm` : wrapper ergonomique SeaORM (find/create/update/delete, scopes), relations (1-1, 1-N, N-N), **factories** + **seeders**.
- [ ] **Migrator agrégé** : collecte des migrations des modules + `database/migrations/`, **tri par timestamp** (DR-021) ; runner.

**Sortie** : test d'intégration sur Postgres (CI) — migration `users` + `roles` + pivot ; create/find/relation OK ; rollback OK.
**Risque** : **élevé** (cœur ergonomique ; tension types Rust ↔ confort Eloquent). Jalon le plus long.

---

## M3 — `afrivel-cli-rt` + `afrivel-cli` (l'outillage)

**But** : générer et piloter un projet.

- [ ] `afrivel-cli-rt` : sous-commandes runtime (clap) `serve`/`migrate*`/`db:seed`/`route:list`, montées dans le binaire `app`.
- [ ] `afrivel-cli` — `new` : scaffolde le **workspace** (`app/`, `modules/`, `config/`, `Afrivel.toml`, `Cargo.toml`, git init).
- [ ] Templates `*.jinja` (`templates/`, embarqués via `rust-embed`) + rendu `minijinja` + helpers.
- [ ] `make:module` : pipeline complet (parsing → rendu → **écriture transactionnelle** rollback → enregistrement workspace/`registry.rs`/`Afrivel.toml` → `rustfmt`).
- [ ] `make:*` granulaires (partagent la fabrique de templates).
- [ ] `dev` : watcher `notify` + debounce + rebuild + restart (garder l'ancien process vivant si build KO).
- [ ] `module:list` (hors-ligne) ; délégation `cargo run -p app -- <cmd>` pour le reste.
- [ ] `completion` (`clap_complete`).

**Sortie** (garde-fou central, DR-014) : test e2e — `new` → `make:module User --model …` → **`cargo build`** en tmpdir = vert ; **golden files** par template.
**Risque** : moyen-élevé (transactionnalité + qualité des templates générés).

---

## M4 — Module Auth (la preuve de valeur)

**But** : un module Auth complet, généré puis raffiné, prêt à l'emploi.

- [ ] Modèles `User`/`Role`/`Permission` + pivots (relations N-N) via l'ORM.
- [ ] Hashing **Argon2** ; register/login ; émission/validation **JWT** ; middleware d'auth.
- [ ] **RBAC** : guards de rôles/permissions (middleware + extracteurs).
- [ ] Routes : `POST /auth/register`, `POST /auth/login`, route protégée d'exemple.
- [ ] Tests du module (unitaires services + intégration HTTP).

**Sortie** : flux register→login→accès protégé vert en test d'intégration.
**Risque** : moyen (dépend de M1–M3 stables).

---

## M5 — App démo + durcissement + release

**But** : prouver l'end-to-end et publier la v0.0.1.

- [ ] `examples/demo` : app réelle utilisant Auth, lancée en CI (`migrate` + smoke test HTTP).
- [ ] Docs utilisateur : quickstart, référence CLI générée, page par module.
- [ ] `cargo install afrivel` vérifié ; binaires pré-compilés (release workflow).
- [ ] Pass `clippy`/`fmt`/audit deps ; vérif licences ; `CHANGELOG.md`.
- [ ] **Tag `v0.0.1`** + publication (crates.io optionnel).

**Sortie** : la « Définition de fini » passe intégralement en CI.

---

## Stratégie de test (transversale)

| Niveau | Cible | Où |
|--------|-------|-----|
| Unitaire | DSL `--model`, mapping types, naming, Error mapping | `afrivel-codegen`, `afrivel-core` |
| Golden files | sortie de chaque template `.jinja` vs `testdata/` | `afrivel-cli` |
| **Compilation réelle** ⭐ | `new`→`make:module`→`cargo build` en tmpdir | `afrivel-cli` (test e2e) |
| Intégration BDD | migrations, CRUD, relations (Postgres en CI) | `afrivel-orm` |
| Intégration HTTP | routes Auth, RBAC, erreurs | module Auth, `examples/demo` |

## Ordre recommandé & parallélisation

- Séquentiel critique : **M0 → M1 → M2 → M3 → M4 → M5**.
- Parallélisable : `afrivel-codegen` (M2) peut démarrer dès M0 ; les **templates** (M3) peuvent être ébauchés pendant M1/M2 ; la **CI** se construit en continu.

## Risques majeurs & mitigations

| Risque | Jalon | Mitigation |
|--------|-------|------------|
| Ergonomie ORM (Eloquent ↔ types Rust) | M2 | Itérer sur l'API publique tôt avec l'app démo ; ne pas viser la « magie », rester explicite. |
| Templates générant du code non-compilable | M3 | Test `cargo build` réel **dès le 1er template** ; golden files. |
| Surface trop large pour v0.0.1 (Auth tire tout) | global | Respecter strictement les non-goals (DESIGN §4) ; pas de scope creep. |
| API `Module`/DI instable | M1 | Geler l'API `Module` avant M3 (la CLI en dépend). |
