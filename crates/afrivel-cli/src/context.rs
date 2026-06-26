//! Construction des contextes de rendu (sérialisés vers minijinja) à partir du DSL `--model`.

use std::time::{SystemTime, UNIX_EPOCH};

use afrivel_codegen::{ModelSpec, naming, types::FieldType};
use serde::Serialize;

/// Version du framework injectée dans les projets générés.
pub const AFRIVEL_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Lignes de dépendances `[dependencies]` vers le framework.
///
/// En mode développement (`AFRIVEL_DEV_PATH` défini → chemin du dossier `crates/`), les
/// projets générés référencent le framework local par chemin ; sinon par version publiée.
/// C'est ce qui permet au test e2e de compiler contre les crates de ce dépôt.
#[derive(Debug, Clone)]
pub struct DepLines {
    pub afrivel: String,
    pub afrivel_rt: String,
}

impl DepLines {
    pub fn resolve() -> Self {
        match std::env::var("AFRIVEL_DEV_PATH") {
            Ok(path) if !path.is_empty() => Self {
                afrivel: format!("afrivel = {{ path = \"{path}/afrivel\" }}"),
                afrivel_rt: format!("afrivel-cli-rt = {{ path = \"{path}/afrivel-cli-rt\" }}"),
            },
            _ => Self {
                afrivel: format!("afrivel = \"{AFRIVEL_VERSION}\""),
                afrivel_rt: format!("afrivel-cli-rt = \"{AFRIVEL_VERSION}\""),
            },
        }
    }
}

/// Référence à un module pour les templates `app/*` (registry, migrator, Cargo.toml).
#[derive(Debug, Clone, Serialize)]
pub struct ModuleRef {
    pub snake: String,
}

/// Contexte des fichiers `app/` régénérés depuis le manifeste.
#[derive(Debug, Clone, Serialize)]
pub struct AppContext {
    pub afrivel_dep: String,
    pub afrivel_rt_dep: String,
    pub modules: Vec<ModuleRef>,
}

/// Contexte du scaffold projet (`new`).
#[derive(Debug, Clone, Serialize)]
pub struct ProjectContext {
    pub name: String,
    pub snake_name: String,
    pub afrivel_version: String,
}

/// Un champ prêt à rendre.
#[derive(Debug, Clone, Serialize)]
pub struct FieldContext {
    pub name: String,
    pub pascal: String,
    pub rust_ty: String,
}

/// Contexte complet d'un module (`make:module`).
#[derive(Debug, Clone, Serialize)]
pub struct ModuleContext {
    pub snake: String,
    pub pascal: String,
    pub plural: String,
    pub table: String,
    pub has_model: bool,
    pub fields: Vec<FieldContext>,
    pub unique_fields: Vec<FieldContext>,
    pub uses_uuid: bool,
    pub uses_decimal: bool,
    pub migration_name: String,
    pub afrivel_dep: String,
}

impl ModuleContext {
    /// Construit le contexte d'un module à partir de son nom et d'une éventuelle `ModelSpec`.
    pub fn build(name: &str, spec: Option<&ModelSpec>, deps: &DepLines) -> Self {
        let pascal = naming::pascal_case(name);
        let snake = naming::snake_case(name);
        let plural = naming::table_name(name);

        let mut fields = Vec::new();
        let mut unique_fields = Vec::new();
        let mut uses_uuid = false;
        let mut uses_decimal = false;

        if let Some(spec) = spec {
            for f in &spec.fields {
                match f.ty {
                    FieldType::Uuid => uses_uuid = true,
                    FieldType::Decimal => uses_decimal = true,
                    _ => {}
                }
                let rust_ty = if f.modifiers.nullable {
                    format!("Option<{}>", f.ty.rust_type())
                } else {
                    f.ty.rust_type().to_string()
                };
                let fc = FieldContext {
                    name: f.name.clone(),
                    pascal: naming::pascal_case(&f.name),
                    rust_ty,
                };
                if f.modifiers.unique {
                    unique_fields.push(fc.clone());
                }
                fields.push(fc);
            }
        }

        let table = spec
            .map(|s| s.table_name())
            .unwrap_or_else(|| plural.clone());
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        Self {
            snake: snake.clone(),
            pascal,
            plural,
            migration_name: format!("m{secs:010}_create_{table}"),
            table,
            has_model: spec.is_some(),
            fields,
            unique_fields,
            uses_uuid,
            uses_decimal,
            afrivel_dep: deps.afrivel.clone(),
        }
    }
}
