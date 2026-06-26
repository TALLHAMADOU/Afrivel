//! Lecture/écriture du manifeste `Afrivel.toml` et localisation de la racine du projet.
//!
//! Le manifeste est la **source de vérité** de la composition : `make:module` l'amende puis
//! régénère `app/Cargo.toml`, `app/src/registry.rs` et `app/src/migrator.rs`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::context::{AppContext, DepLines, ModuleRef};
use crate::ui::CliError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub project: Project,
    #[serde(default)]
    pub modules: BTreeMap<String, ModuleEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub afrivel_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleEntry {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl Manifest {
    /// Remonte l'arborescence depuis `start` jusqu'à trouver un `Afrivel.toml`.
    pub fn find_root(start: &Path) -> Option<PathBuf> {
        let mut dir = Some(start);
        while let Some(d) = dir {
            if d.join("Afrivel.toml").is_file() {
                return Some(d.to_path_buf());
            }
            dir = d.parent();
        }
        None
    }

    /// Charge le manifeste depuis la racine du projet.
    pub fn load(root: &Path) -> Result<Self, CliError> {
        let raw = std::fs::read_to_string(root.join("Afrivel.toml")).map_err(|e| {
            CliError::user(format!(
                "✗ Pas un projet Afrivel — {e} → lance `afrivel new`"
            ))
        })?;
        toml::from_str(&raw)
            .map_err(|e| CliError::internal(format!("Afrivel.toml illisible — {e}")))
    }

    /// Sérialise le manifeste vers la racine du projet.
    pub fn save(&self, root: &Path) -> Result<(), CliError> {
        let body = toml::to_string_pretty(self)
            .map_err(|e| CliError::internal(format!("sérialisation Afrivel.toml — {e}")))?;
        std::fs::write(root.join("Afrivel.toml"), body).map_err(CliError::from)
    }

    /// Contexte de régénération des fichiers `app/` (ordre déterministe par nom).
    pub fn app_context(&self, deps: &DepLines) -> AppContext {
        AppContext {
            afrivel_dep: deps.afrivel.clone(),
            afrivel_rt_dep: deps.afrivel_rt.clone(),
            modules: self
                .modules
                .keys()
                .map(|snake| ModuleRef {
                    snake: snake.clone(),
                })
                .collect(),
        }
    }
}
