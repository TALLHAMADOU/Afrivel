# Documentation de développement — Afrivel

Ces documents sont destinés aux **contributeurs** du framework. Pour la présentation du framework côté utilisateur, voir le [`README.md`](../README.md) racine.

| Document | Contenu |
|----------|---------|
| [DESIGN.md](./DESIGN.md) | Understanding summary, hypothèses, NFR, périmètre v0.0.1, risques. |
| [DECISIONS.md](./DECISIONS.md) | Decision Log (DR-001 → DR-014 + décisions en attente). |
| [ARCHITECTURE.md](./ARCHITECTURE.md) | Frontière Go↔Rust, layout projet, anatomie module, double registre, crates. |
| [CLI.md](./CLI.md) | Spécification complète de la CLI : commandes, codegen `make:module`, erreurs, dev loop, tests. |
| [ROADMAP.md](./ROADMAP.md) | Périmètre v0.0.1 (checklist) + post-v0.0.1 + décisions à trancher. |

## État du projet

**Phase : design validé, avant implémentation.** Issu d'une session de brainstorming structuré (compréhension verrouillée et confirmée, design accepté section par section).

## Décisions structurantes (résumé)

- Couche de productivité au-dessus d'**Axum + Tower + Tokio** (pas un serveur HTTP de plus).
- ORM ergonomique au-dessus de **SeaORM/sqlx**.
- **CLI en Go + Cobra** = orchestrateur ; runtime en **crates Rust** ; délégation à `cargo run` pour tout ce qui touche la BDD.
- Enregistrement modules/routes **explicite** (manifeste TOML + `mod.rs`), **sans réflexion runtime**.
- Codegen **transactionnel**, sortie **toujours compilable** (garde-fou : test `cargo build` réel).
- Auto-reload (watch/recompile/restart), **pas** de hot-swap.

## Points ouverts

Licence, édition Rust (stable/nightly), et angle de différenciation vs **Loco.rs** — voir [ROADMAP.md](./ROADMAP.md).
