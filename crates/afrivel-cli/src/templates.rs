//! Moteur de templates : `*.jinja` embarqués (`include_str!`) rendus via `minijinja`.
//!
//! Source unique des artefacts générés (scaffold de projet + modules). Les golden tests
//! vérifient la stabilité du rendu ; le test e2e vérifie qu'il **compile**.

pub use minijinja::Environment;
use serde::Serialize;

use crate::ui::CliError;

/// `(nom logique, contenu)` de chaque template embarqué.
pub const TEMPLATES: &[(&str, &str)] = &[
    (
        "project/Cargo.toml",
        include_str!("../../../templates/project/Cargo.toml.jinja"),
    ),
    (
        "project/Afrivel.toml",
        include_str!("../../../templates/project/Afrivel.toml.jinja"),
    ),
    (
        "project/gitignore",
        include_str!("../../../templates/project/gitignore.jinja"),
    ),
    (
        "project/config/default.toml",
        include_str!("../../../templates/project/config/default.toml.jinja"),
    ),
    (
        "project/app/Cargo.toml",
        include_str!("../../../templates/project/app/Cargo.toml.jinja"),
    ),
    (
        "project/app/main.rs",
        include_str!("../../../templates/project/app/main.rs.jinja"),
    ),
    (
        "project/app/registry.rs",
        include_str!("../../../templates/project/app/registry.rs.jinja"),
    ),
    (
        "project/app/migrator.rs",
        include_str!("../../../templates/project/app/migrator.rs.jinja"),
    ),
    (
        "module/Cargo.toml",
        include_str!("../../../templates/module/Cargo.toml.jinja"),
    ),
    (
        "module/lib.rs",
        include_str!("../../../templates/module/lib.rs.jinja"),
    ),
    (
        "module/routes.rs",
        include_str!("../../../templates/module/routes.rs.jinja"),
    ),
    (
        "module/model.rs",
        include_str!("../../../templates/module/model.rs.jinja"),
    ),
    (
        "module/repository.rs",
        include_str!("../../../templates/module/repository.rs.jinja"),
    ),
    (
        "module/controller.rs",
        include_str!("../../../templates/module/controller.rs.jinja"),
    ),
    (
        "module/migration.rs",
        include_str!("../../../templates/module/migration.rs.jinja"),
    ),
];

/// Construit l'environnement minijinja chargé de tous les templates.
pub fn environment() -> Environment<'static> {
    let mut env = Environment::new();
    for (name, src) in TEMPLATES {
        env.add_template(name, src)
            .expect("template embarqué valide");
    }
    env
}

/// Rend un template par son nom logique avec le contexte fourni.
pub fn render<S: Serialize>(
    env: &Environment<'static>,
    name: &str,
    ctx: S,
) -> Result<String, CliError> {
    let tmpl = env
        .get_template(name)
        .map_err(|e| CliError::internal(format!("template `{name}` introuvable — {e}")))?;
    tmpl.render(ctx)
        .map_err(|e| CliError::internal(format!("rendu de `{name}` — {e}")))
}
