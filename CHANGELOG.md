# Changelog

Toutes les évolutions notables de ce projet sont documentées ici. Le format suit
[Keep a Changelog](https://keepachangelog.com/fr/1.1.0/) et le projet adhère au
[versioning sémantique](https://semver.org/lang/fr/).

## [Unreleased]

## [0.0.1] — 2026-06-28

Première version : socle du framework, outillage CLI et module Auth de référence prouvé
end-to-end.

### Added

- **Noyau** (`afrivel-core`) : `Application` (registre de modules + DI par `provide`/Extension),
  trait `Module`, `Error → IntoResponse`, configuration (serde + figment), logs `tracing`,
  validation (`Validate`, `ValidatedJson`).
- **Auth (primitives)** sous `afrivel::auth` : hachage Argon2id, JWT HS256
  (`JwtSecret`/`Claims`/`encode`/`decode`), extracteur `AuthUser`, middleware `authenticate`,
  RBAC typé `Guard` + `Authorized<G>`.
- **ORM** (`afrivel-orm` + `afrivel-macros`) : persistance SeaORM ergonomique, `migrator::sorted`
  (ordre déterministe par timestamp), `db_error` (mapping `DbErr → Error`), derive `Model`.
- **CLI** (`afrivel-cli`, binaire `afrivel`) : `new`, `make:module`, `dev` (auto-reload),
  délégation runtime via `afrivel-cli-rt` (`serve`, `migrate*`, `db:seed`, `route:list`).
- **Façade** (`afrivel`) : ré-exports `afrivel_core::*`, `orm`, `tokio`, `axum`, macro `Model`.
- **App démo** (`examples/demo`) : module Auth complet en Clean Architecture (User/Role/Permission
  + pivots N-N, register/login, routes `/auth/{register,login,me,admin}`, `AdminGuard`),
  dépôts InMemory et SeaORM.
- **Docs** : quickstart, référence CLI, page module Auth, architecture, décisions, roadmap.

### Tested

- Compilation réelle `new → make:module → cargo build` (garde-fou e2e CLI).
- Auth : tests unitaires `AuthService` + intégration HTTP (register→login→protégé, RBAC, validation).
- Smoke end-to-end sur **Postgres** en CI (migrations + flux HTTP).

### CI / Release

- Pipeline `fmt · clippy · test` (Linux/macOS/Windows), job d'intégration Postgres, `cargo audit`.
- Workflow de release : binaires `afrivel` pré-compilés sur tag `vX.Y.Z`.

[Unreleased]: https://github.com/TALLHAMADOU/Afrivel/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/TALLHAMADOU/Afrivel/releases/tag/v0.0.1
