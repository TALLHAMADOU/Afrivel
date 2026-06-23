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
| `new <nom>` | Crée un projet complet (layout, `Afrivel.toml`, `Cargo.toml`, `src/bin/afrivel.rs`, git init). |
| `version` / `--version` | Version CLI + runtime attendu. |
| `completion <shell>` | Complétion bash/zsh/fish (natif Cobra). |

### Génération (pur Go, codegen `go:embed`)
| Commande | Rôle |
|----------|------|
| `make:module <Nom> [--model …]` | ⭐ Module complet (+ CRUD si `--model`). |
| `make:model <Nom>` | Modèle dans un module existant. |
| `make:controller`, `make:service`, `make:repository`, `make:request`, `make:resource`, `make:migration`, `make:test`, `make:middleware` | Composant granulaire. |
| `make:seeder`, `make:factory` | Données de test. |

> `make:module` = orchestration des générateurs granulaires (même fabrique de templates, DRY).

### Runtime (délégué à `cargo run`)
| Commande | Rôle |
|----------|------|
| `serve [--port]` | Lance l'app. |
| `dev` | watch + recompile + restart. |
| `migrate` / `migrate:rollback` / `migrate:fresh` / `migrate:status` | Migrations (SeaORM). |
| `db:seed` | Exécute les seeders. |

### Introspection (pur Go, lit le manifeste)
| Commande | Rôle |
|----------|------|
| `module:list` | Modules enregistrés. |
| `route:list` | Table des routes. |

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
5. **Enregistrement** : met à jour `Afrivel.toml [modules]` + `modules/mod.rs` (+ registre de migrations si `--model`).
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
| Manifeste ↔ `mod.rs` divergents | `warn` + `afrivel doctor` (post-v0.0.1). |

---

## Boucle `afrivel dev`

```
1. Watcher (fsnotify) : src/, modules/, config/, routes/
2. Debounce ~300ms
3. cargo build (délégué) ; échec → affiche erreurs, garde l'ancien process VIVANT
4. Succès → SIGTERM gracieux à l'ancien → lance le nouveau
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
