# Afrivel CLI — Spécification

Convention de nommage : **`namespace:action`** (familier artisan/Laravel). Implémentation Go + Cobra.

## Flags globaux

| Flag | Effet |
|------|-------|
| `--quiet` | Silencieux (erreurs seules). |
| `--verbose` | Détails + stacktrace Go (debug). |
| `--force` | Écrase les fichiers existants. |
| `--dry-run` | Affiche l'arbre/diffs, **n'écrit rien**. |
| `--no-manifest` | Échappatoire : ne touche pas `Afrivel.toml`. |

## Commandes

### Bootstrap & projet (pur Go)
| Commande | Rôle |
|----------|------|
| `new <nom>` | Crée un projet complet (workspace : `Cargo.toml [workspace]`, `Afrivel.toml`, crate `app/` avec `main.rs`/`registry.rs`/`migrator.rs`, git init). |
| `version` / `--version` | Version CLI + runtime attendu. |
| `completion <shell>` | Complétion bash/zsh/fish (natif Cobra). |

### Génération (pur Go, codegen `go:embed`)
| Commande | Rôle |
|----------|------|
| `make:module <Nom> [--model …] [--depends a,b]` | ⭐ Module complet (+ CRUD si `--model`). `--depends` câble les dépendances inter-modules (path-dep + `contracts`). |
| `make:model <Nom>` | Modèle dans un module existant. |
| `make:controller`, `make:service`, `make:repository`, `make:request`, `make:resource`, `make:migration`, `make:test`, `make:middleware` | Composant granulaire. |
| `make:seeder`, `make:factory` | Données de test. |

> `make:module` = orchestration des générateurs granulaires (même fabrique de templates, DRY).

### Runtime (délégué à `cargo run -p app`)
| Commande | Rôle |
|----------|------|
| `serve [--port]` | Lance l'app. |
| `dev` | watch + recompile + restart. |
| `migrate` / `migrate:rollback` / `migrate:fresh` / `migrate:status` | Migrations (SeaORM, ordre par timestamp). |
| `db:seed` | Exécute les seeders. |
| `route:list` | Table des routes — **délégué** : seul le runtime, qui monte le routeur Axum, connaît les routes (elles ne sont pas dans le manifeste). |

### Introspection (pur Go, lit le manifeste)
| Commande | Rôle |
|----------|------|
| `module:list` | Modules enregistrés (depuis `Afrivel.toml`). |

### Réservé (post-v0.0.1, non implémenté)
`make:job`, `make:event`, `make:command`, `queue:work`, `schedule:run`, `tinker`, `doctor`.

---

## `make:module` — Pipeline de codegen

1. **Parsing**
   - Nom → PascalCase, unicité (manifeste).
   - `--model` mini-DSL : `Nom:champ:type[:modificateur][,…]`.
     - Types : `string, int, bool, text, timestamp, uuid, decimal, fk:Model`.
     - Modificateurs : `unique`, `nullable`, `default=…`.
     - Ex. : `User:name:string,email:string:unique,bio:text:nullable,price:decimal:default=0`.
   - Sortie : struct `ModuleSpec`.
2. **Résolution templates** : `.tmpl` embarqués (`model.rs.tmpl`, `controller.rs.tmpl`, `repository.rs.tmpl`, `service.rs.tmpl`, `request.rs.tmpl`, `resource.rs.tmpl`, `migration.rs.tmpl`, `routes.rs.tmpl`, `test.rs.tmpl`, `mod.rs.tmpl`).
3. **Rendu** : `ModuleSpec` + helpers (`snake_case`, `plural`, `table_name`, mapping de types). Sans `--model` → squelettes vides **compilables** ; avec → CRUD câblé.
4. **Écriture transactionnelle** : `--dry-run` n'écrit rien ; collisions → `--force` requis ; **rollback** si échec partiel.
5. **Enregistrement** (workspace) :
   - crée la crate `modules/<nom>/` (avec `Cargo.toml`) ;
   - ajoute le membre au `[workspace]` racine + la path-dep dans `app/Cargo.toml` ;
   - câble `app/src/registry.rs` (`<nom>::module()`) et, si `--model`, la migration (préfixe timestamp) agrégée par `app/src/migrator.rs` ;
   - met à jour `Afrivel.toml [modules.<nom>]` (path, model, deps) ;
   - si `--depends` : path-deps inter-modules + `deps = [...]` dans le manifeste.
6. **Post** : `rustfmt` (si dispo, sinon `warn`) ; récap des fichiers + prochaine étape (`afrivel migrate`).

**Garantie** : sortie toujours compilable.

---

## Gestion des erreurs

- Niveaux : `error` (exit≠0), `warn` (continue), `hint` (suggestion).
- Format : `✗ <quoi> — <pourquoi> → <comment corriger>`.
- Codes de sortie : `0` ok, `1` erreur utilisateur, `2` erreur interne. Stacktrace Go seulement sous `--verbose`.

### Edge cases
| Cas | Comportement |
|-----|--------------|
| Commande déléguée hors projet | `✗ Pas un projet Afrivel (Afrivel.toml absent) → lance 'afrivel new'`. |
| `cargo`/`rustc` absent | Détecté tôt + lien install. |
| Nom = mot-clé Rust | Rejeté à la validation. |
| `--model` malformé | Pointe le token fautif + syntaxe attendue. |
| Collision partielle | Liste précise + `--force` ; rollback si interrompu. |
| `rustfmt` absent | `warn`, on continue (templates pré-formatés). |
| Manifeste ↔ `registry.rs`/`Cargo.toml` divergents | `warn` + `afrivel doctor` (post-v0.0.1) ; le code Rust fait foi. |
| `--depends` sur module inexistant | Rejeté avant écriture (le module cible doit exister dans le manifeste). |
| Mismatch `afrivel_version` (CLI Go ↔ runtime) | `warn` clair : contrat de sous-commandes potentiellement divergent. |

---

## Boucle `afrivel dev`

```
1. Watcher (fsnotify) : app/src/, modules/*/src/, config/, tests/
2. Debounce ~300ms
3. cargo build -p app (délégué) ; échec → affiche erreurs, garde l'ancien process VIVANT
4. Succès → SIGTERM gracieux à l'ancien → lance le nouveau (cargo run -p app -- serve)
5. Logs unifiés + statut (⟳ building / ✓ running :port)
```

- Ignore `target/`, `storage/`, `.git/`, temp → évite la boucle infinie.
- Filtrage : `.rs` → rebuild complet ; `config/*.toml` & assets → reload à chaud sans rebuild.
- Port occupé → message + `--port`. Panic runtime → stack affichée, **pas** de rebuild en boucle (attend un changement).

---

## Stratégie de test (CLI Go)

1. **Unitaires** : parser DSL `--model`, helpers casse/pluralisation, mapping de types, validation de noms.
2. **Golden files** : sortie de chaque template vs `testdata/` de référence.
3. **Compilation réelle** ⭐ : `new` → `make:module --model …` → `cargo build` en tmpdir → assert compile. Garde-fou central.
4. **Idempotence/transaction** : interruption sans orphelins ; `--dry-run` n'écrit rien.
5. **CI matrix** : Linux/macOS/Windows × Rust stable.
