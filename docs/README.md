# Documentation de développement — Afrivel

Ces documents sont destinés aux **contributeurs** du framework. Pour la présentation du framework côté utilisateur, voir le [`README.md`](../README.md) racine.

| Document | Contenu |
|----------|---------|
| [QUICKSTART.md](./QUICKSTART.md) | Prise en main : `new`, `make:module`, migrations, `serve`, flux Auth. |
| [modules/auth.md](./modules/auth.md) | Module Auth de référence : primitives `afrivel::auth`, couches, routes, RBAC. |
| [DESIGN.md](./DESIGN.md) | Understanding summary, hypothèses, NFR, périmètre v0.0.1, risques. |
| [DECISIONS.md](./DECISIONS.md) | Decision Log (DR-001 → DR-014 + décisions en attente). |
| [ARCHITECTURE.md](./ARCHITECTURE.md) | CLI globale ↔ app, layout workspace, anatomie module, double registre, crates. |
| [CLI.md](./CLI.md) | Spécification complète de la CLI : commandes, codegen `make:module`, erreurs, dev loop, tests. |
| [ROADMAP.md](./ROADMAP.md) | Périmètre v0.0.1 (checklist) + post-v0.0.1 + décisions à trancher. |
| [IMPLEMENTATION.md](./IMPLEMENTATION.md) | Plan d'implémentation séquencé (M0→M5), critères de sortie, tests, risques. |

## État du projet

**Phase : design validé, avant implémentation.** Issu d'une session de brainstorming structuré (compréhension verrouillée et confirmée, design accepté section par section).

## Décisions structurantes (résumé)

- Couche de productivité au-dessus d'**Axum + Tower + Tokio** (pas un serveur HTTP de plus).
- ORM ergonomique au-dessus de **SeaORM/sqlx** ; migrations ordonnées par timestamp.
- Projet généré = **Cargo workspace** ; chaque module = **crate** en **Clean Architecture** (couches `http → services → contracts ← repositories`).
- **CLI unique en Rust (clap)** : binaire global `afrivel` (`cargo install`) ; runtime délégué au binaire `app` du projet via `cargo run -p app` (BDD **et** `route:list`). Mono-langage, contrat partagé via `afrivel-cli-rt`.
- Double registre **explicite** : `Afrivel.toml` (outillage) + `app/src/registry.rs`/`Cargo.toml` (compilation), **sans réflexion runtime** ; le code Rust fait foi.
- Gestion d'erreurs unifiée (`afrivel::Error → IntoResponse`), config serde+figment, logs `tracing`.
- Codegen **transactionnel**, sortie **toujours compilable** (garde-fou : test `cargo build` réel).
- Auto-reload (watch/recompile/restart), **pas** de hot-swap.

## Points ouverts

Aucun bloquant. Tous les points de design v0.0.1 sont tranchés (DR-001 → DR-024), y compris licence (`MIT OR Apache-2.0`), toolchain (Rust stable, edition 2024, MSRV 1.85+) et différenciation vs **Loco.rs** (module-centric + Clean Architecture). Voir [DECISIONS.md](./DECISIONS.md).
