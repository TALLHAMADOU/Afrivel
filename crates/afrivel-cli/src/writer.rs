//! Écriture transactionnelle d'un ensemble de fichiers générés.
//!
//! Garanties (cf. docs/CLI.md) : `--dry-run` n'écrit rien ; collision sur un fichier *neuf*
//! → erreur listant les conflits (sauf `--force`) ; en cas d'échec d'écriture, les fichiers
//! nouvellement créés sont supprimés (rollback). Les fichiers *managés* (régénérés depuis le
//! manifeste : `app/Cargo.toml`, `registry.rs`, `migrator.rs`) sont écrasés sans `--force`.

use std::fs;
use std::path::{Path, PathBuf};

use crate::ui::{CliError, CliResult, Ui};

struct Entry {
    path: PathBuf,
    content: String,
    /// Écrasement autorisé sans `--force` (fichier régénéré, source = manifeste).
    managed: bool,
}

/// Plan d'écriture appliqué de façon transactionnelle sous une racine donnée.
pub struct Plan {
    root: PathBuf,
    entries: Vec<Entry>,
}

impl Plan {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            entries: Vec::new(),
        }
    }

    /// Ajoute un fichier *neuf* (collision contrôlée).
    pub fn add(&mut self, rel: impl AsRef<Path>, content: impl Into<String>) {
        self.entries.push(Entry {
            path: self.root.join(rel),
            content: content.into(),
            managed: false,
        });
    }

    /// Ajoute un fichier *managé* (écrasé sans `--force`).
    pub fn add_managed(&mut self, rel: impl AsRef<Path>, content: impl Into<String>) {
        self.entries.push(Entry {
            path: self.root.join(rel),
            content: content.into(),
            managed: true,
        });
    }

    /// Applique le plan. `dry_run` n'écrit rien ; `force` ignore les collisions.
    pub fn apply(&self, ui: &Ui, force: bool, dry_run: bool) -> CliResult {
        // 1. Détecte les collisions sur les fichiers neufs.
        let collisions: Vec<&PathBuf> = self
            .entries
            .iter()
            .filter(|e| !e.managed && !force && e.path.exists())
            .map(|e| &e.path)
            .collect();
        if !collisions.is_empty() {
            let list = collisions
                .iter()
                .map(|p| format!("  - {}", p.display()))
                .collect::<Vec<_>>()
                .join("\n");
            return Err(CliError::user(format!(
                "✗ Collision — ces fichiers existent déjà :\n{list}\n→ relance avec --force pour écraser"
            )));
        }

        // 2. Mode plan : affiche l'arbre, n'écrit rien.
        if dry_run {
            ui.info("Plan (--dry-run, aucune écriture) :");
            for e in &self.entries {
                let tag = if e.path.exists() {
                    "écrase "
                } else {
                    "crée   "
                };
                ui.info(format!("  {tag} {}", e.path.display()));
            }
            return Ok(());
        }

        // 3. Écriture avec rollback des fichiers nouvellement créés.
        let mut created: Vec<PathBuf> = Vec::new();
        for e in &self.entries {
            let existed = e.path.exists();
            if let Err(err) = write_file(&e.path, &e.content) {
                rollback(&created, ui);
                return Err(CliError::internal(format!(
                    "✗ Écriture de {} — {err} (rollback effectué)",
                    e.path.display()
                )));
            }
            if !existed {
                created.push(e.path.clone());
            }
            ui.detail(format!("écrit {}", e.path.display()));
        }
        Ok(())
    }
}

fn write_file(path: &Path, content: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

fn rollback(created: &[PathBuf], ui: &Ui) {
    for p in created.iter().rev() {
        let _ = fs::remove_file(p);
    }
    if !created.is_empty() {
        ui.warn("écriture interrompue : fichiers créés annulés");
    }
}
