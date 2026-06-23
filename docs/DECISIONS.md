# Afrivel — Decision Log

Chaque décision : **ce qui est décidé**, **alternatives**, **pourquoi**.

---

### DR-001 — Objectif : framework OSS sérieux
- **Décidé** : viser un vrai framework destiné à l'adoption externe.
- **Alternatives** : MVP/PoC, projet d'apprentissage, outil interne perso.
- **Pourquoi** : ambition portée par la vision ; impose dès le départ stabilité d'API, docs, tests, CI.

### DR-002 — Fondation : Axum + Tower + Tokio
- **Décidé** : bâtir au-dessus d'Axum/Tower/Tokio.
- **Alternatives** : Actix-web ; from scratch sur hyper ; tout from scratch.
- **Pourquoi** : stack la plus mature, écosystème middleware Tower, time-to-value. Afrivel = couche d'ergonomie, pas un serveur HTTP de plus.

### DR-003 — ORM au-dessus de SeaORM/sqlx
- **Décidé** : s'appuyer sur SeaORM/sqlx ; ajouter migrations, factories, seeders, relations, scopes ergonomiques.
- **Alternatives** : Active Record custom via macros lourdes ; query builder + codegen pur.
- **Pourquoi** : l'Active-Record « magique » d'Eloquent se heurte au système de types/ownership de Rust ; réimplémenter un ORM = années de R&D. SeaORM offre l'async + un socle relationnel solide.

### DR-004 — Cible : pont équitable Laravel ↔ Rust
- **Décidé** : servir à parts égales devs Laravel migrants et Rustaceans.
- **Alternatives** : prioriser migrants PHP ; prioriser Rustaceans ; startups time-to-market.
- **Pourquoi** : positionnement le plus différenciant. **Coût** : tension de design permanente, arbitrée cas par cas.

### DR-005 — Enregistrement modules/routes : explicite, assisté par CLI
- **Décidé** : manifeste + registre Rust maintenus par la CLI ; pas de réflexion runtime.
- **Alternatives** : registre compile-time (linkme/inventory) ; génération via build.rs.
- **Pourquoi** : 100 % prévisible, debuggable, robuste, sans magie linker fragile — cohérent avec un framework « sérieux ».

### DR-006 — Périmètre v0.0.1 : Noyau + ORM + Auth complet
- **Décidé** : v0.0.1 inclut Core, ORM relationnel, migrations, CRUD, et module Auth complet (JWT/RBAC/permissions).
- **Alternatives** : noyau+CRUD seul ; noyau+ORM riche ; tout le Core listé.
- **Pourquoi** : choix utilisateur. **Implication assumée** : Auth tire Core + ORM relationnel + middleware → jalon conséquent.

### DR-007 — Auto-reload, pas hot-swap
- **Décidé** : watch + recompile + restart auto (« auto-reload »). Retirer le terme « hot reload ».
- **Alternatives** : reload assets/templates à chaud ; hot-swap dylib ; abandonner la promesse.
- **Pourquoi** : Rust est compilé ; le hot-swap est fragile/expérimental. On reste honnête. Reload à chaud des assets/config non-compilés conservé comme bonus.

### DR-008 — CLI en Go + Cobra (runtime en Rust)
- **Décidé** : CLI `afrivel` développée en Go avec Cobra ; framework runtime en crates Rust.
- **Alternatives** : CLI en Rust (clap).
- **Pourquoi** : choix utilisateur. Cobra = excellente base (sous-commandes, flags, aide, complétion shell), binaire statique multi-OS. **Coût** : architecture bi-langage → frontière stricte requise (DR-009).

### DR-009 — CLI Go = orchestrateur ; runtime délégué à `cargo`
- **Décidé** : la CLI Go fait scaffolding/codegen/watch ; délègue `migrate/serve/seed` à `cargo run` sur `src/bin/afrivel.rs`.
- **Alternatives** : binaire Rust compagnon pré-compilé ; drivers SQL Go natifs.
- **Pourquoi** : source de vérité BDD unique (SeaORM), pas de double pile de drivers, frontière nette. La CLI ne touche jamais la BDD ni ne parse du Rust.

### DR-010 — Templates de codegen : `text/template` + `go:embed`
- **Décidé** : templates embarqués dans le binaire Go via `go:embed`, rendus avec `text/template`.
- **Alternatives** : fichiers templates externes ; codegen Rust pur.
- **Pourquoi** : conséquence naturelle de DR-008 ; distribution mono-binaire, pas de dépendances runtime côté templates.

### DR-011 — Codegen transactionnel + sortie toujours compilable
- **Décidé** : écriture atomique (rollback si échec partiel), `--dry-run`/`--force`, `rustfmt` post-génération ; squelettes vides **compilables**.
- **Pourquoi** : un `make:*` qui laisse un module à moitié généré ou non-compilable détruit le « wow » Laravel.

### DR-012 — Double registre : manifeste TOML + `mod.rs` ; code = vérité
- **Décidé** : `Afrivel.toml` = vérité **outillage** (CLI) ; `modules/mod.rs` = vérité **compilation**. En cas de divergence, le **code Rust fait foi** ; le manifeste est régénérable.
- **Pourquoi** : la CLI Go lit/écrit le TOML sans parser du Rust (respect de la frontière DR-009), tout en gardant la compilation comme autorité finale.

### DR-013 — Boucle dev : garder l'ancien process vivant pendant le build
- **Décidé** : sur changement, rebuild ; ne tuer l'ancien serveur **qu'après** un build réussi ; filtrage rebuild (`.rs`) vs reload (config/assets).
- **Pourquoi** : zéro fenêtre « down » sur erreur de compilation ; UX de logs unifiée.

### DR-014 — Garde-fou test : compilation réelle
- **Décidé** : test d'intégration `new` → `make:module --model` → **`cargo build`** en tmpdir ; + golden files par template ; CI matrix OS × Rust.
- **Pourquoi** : seul moyen fiable de garantir « toujours compilable » et de détecter les régressions de codegen.

### DR-015 — Licence : `MIT OR Apache-2.0` (dual)
- **Décidé** : double licence MIT + Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`).
- **Alternatives** : MIT seule ; Apache-2.0 seule.
- **Pourquoi** : standard de l'écosystème Rust ; compatibilité maximale + clause brevets (Apache).

### DR-016 — Toolchain : Rust stable, edition 2024
- **Décidé** : framework utilisable sur **Rust stable**, **edition 2024** → **MSRV 1.85+**. Aucune feature nightly requise.
- **Alternatives** : stable edition 2021 ; nightly autorisé.
- **Pourquoi** : edition 2024 = syntaxe moderne, et stable = adoption sérieuse + CI simple. Coût assumé : exclut les toolchains < 1.85.

### DR-017 — Différenciation vs Loco.rs : Module-centric + Clean Architecture
- **Décidé** : positionnement = **tranches verticales autonomes** (`make:module`) + **couches imposées par défaut** (Services / Repositories / Interfaces / Resources). Cible : apps maintenables, en équipe, à grande échelle.
- **Alternatives** : DX bilingue Laravel-first ; scaffolding le plus complet ; angle robustesse/explicite.
- **Pourquoi** : Loco vise le prototypage rapide façon Rails (composants fins, individuels). Afrivel occupe le créneau « Laravel Modules + Clean Architecture » — architecturalement distinct, aligné avec la fonctionnalité signature.

---

## Décisions en attente
_Aucune — tous les points ouverts du design v0.0.1 sont tranchés._
