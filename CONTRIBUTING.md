# Contribuer à Afrivel

Merci d'envisager de contribuer à Afrivel ! Le projet accueille toutes les contributions :
code, documentation, tests, traductions, idées et retours.

> ⚠️ **Statut : v0.0.1 en cours de design.** Le code du framework n'existe pas encore.
> La contribution la plus utile aujourd'hui porte sur le **design** et la **documentation**.

## Avant de commencer

1. Lis la documentation de conception dans [`/docs`](./docs/README.md) :
   - [DESIGN.md](./docs/DESIGN.md) — vision, hypothèses, périmètre.
   - [ARCHITECTURE.md](./docs/ARCHITECTURE.md) — workspace, modules, Clean Architecture.
   - [DECISIONS.md](./docs/DECISIONS.md) — **Decision Log** : toute décision structurante y est tracée.
   - [CLI.md](./docs/CLI.md) — spécification de la CLI.
   - [ROADMAP.md](./docs/ROADMAP.md) — feuille de route v0.0.1.
2. Ouvre une **issue** pour discuter d'un changement non trivial **avant** d'écrire du code.
   Une proposition qui contredit une décision du Decision Log doit le mentionner explicitement.

## Principes d'architecture (non négociables pour la v0.0.1)

- **Tout en Rust** (mono-langage) — CLI comprise (`clap`).
- **Pas de réflexion runtime** : enregistrement des modules/routes explicite.
- Projet généré = **Cargo workspace** ; chaque module = **crate** en **Clean Architecture**
  (`http → services → contracts ← repositories`, domaine sans dépendance infra).
- **Toute sortie de `make:*` doit compiler** (`cargo build` vert) — garde-fou central.

## Flux de contribution

1. Forke le dépôt et crée une branche depuis `main` :
   `git checkout -b feat/ma-fonctionnalite`.
2. Fais des commits clairs et atomiques.
3. Assure-toi que le projet **compile** et que les **tests passent** (une fois le code en place) :
   ```bash
   cargo fmt --all
   cargo clippy --all-targets -- -D warnings
   cargo test --workspace
   ```
4. Ouvre une **Pull Request** vers `main` en décrivant le *quoi* et le *pourquoi*, et en
   référençant l'issue associée.

## Style de code

- `rustfmt` (configuration par défaut) et `clippy` sans warning.
- Rust **stable**, edition **2024** (MSRV 1.85+).
- Pas de `unsafe` sans justification documentée.

## Toolchain

- Rust stable ≥ 1.85 (`rustup toolchain install stable`).
- Postgres (pour les tests d'intégration ORM/migrations) — v0.0.1.

## Licence des contributions

En contribuant, tu acceptes que tes contributions soient distribuées sous la **double licence
MIT OR Apache-2.0** du projet (voir [`LICENSE-MIT`](./LICENSE-MIT) / [`LICENSE-APACHE`](./LICENSE-APACHE)).

## Code de conduite

Toute participation est soumise au [Code de conduite](./CODE_OF_CONDUCT.md).
