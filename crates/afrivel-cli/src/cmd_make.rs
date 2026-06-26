//! `afrivel make:module <Nom> [--model …]` — pipeline de génération d'un module.
//!
//! Parsing → rendu → écriture transactionnelle → amendement du manifeste → régénération des
//! fichiers `app/` managés → `rustfmt`. Garantie : sortie compilable (test e2e).

use std::path::Path;
use std::process::Command;

use afrivel_codegen::{ModelSpec, naming};

use crate::cli::Globals;
use crate::context::{DepLines, ModuleContext};
use crate::manifest::{Manifest, ModuleEntry};
use crate::templates::{Environment, render};
use crate::ui::{CliError, CliResult, Ui};
use crate::writer::Plan;

pub fn run(
    ui: &Ui,
    env: &Environment<'static>,
    g: &Globals,
    name: &str,
    model: Option<&str>,
) -> CliResult {
    let cwd = std::env::current_dir()?;
    let root = Manifest::find_root(&cwd).ok_or_else(|| {
        CliError::user("✗ Pas un projet Afrivel (Afrivel.toml absent) → lance `afrivel new`")
    })?;

    let mut manifest = Manifest::load(&root)?;

    let spec = match model {
        Some(m) => Some(ModelSpec::parse(&format!("{name}:{m}"))?),
        None => None,
    };
    let snake = naming::snake_case(name);
    let pascal = naming::pascal_case(name);

    if manifest.modules.contains_key(&snake) && !g.force {
        return Err(CliError::user(format!(
            "✗ Le module `{snake}` existe déjà dans le manifeste → choisis un autre nom ou --force"
        )));
    }

    let deps = DepLines::resolve();
    let ctx = ModuleContext::build(name, spec.as_ref(), &deps);

    // Amende le manifeste en mémoire (utilisé pour régénérer les fichiers `app/`).
    manifest.modules.insert(
        snake.clone(),
        ModuleEntry {
            path: format!("modules/{snake}"),
            model: model.map(str::to_string),
        },
    );
    let app = manifest.app_context(&deps);

    // Fichiers du module (neufs) + fichiers `app/` régénérés (managés).
    let mut plan = Plan::new(&root);
    let base = format!("modules/{snake}");
    plan.add(
        format!("{base}/Cargo.toml"),
        render(env, "module/Cargo.toml", &ctx)?,
    );
    plan.add(
        format!("{base}/src/lib.rs"),
        render(env, "module/lib.rs", &ctx)?,
    );
    plan.add(
        format!("{base}/src/routes.rs"),
        render(env, "module/routes.rs", &ctx)?,
    );
    if spec.is_some() {
        plan.add(
            format!("{base}/src/model.rs"),
            render(env, "module/model.rs", &ctx)?,
        );
        plan.add(
            format!("{base}/src/repository.rs"),
            render(env, "module/repository.rs", &ctx)?,
        );
        plan.add(
            format!("{base}/src/controller.rs"),
            render(env, "module/controller.rs", &ctx)?,
        );
        plan.add(
            format!("{base}/src/migration.rs"),
            render(env, "module/migration.rs", &ctx)?,
        );
    }
    plan.add_managed(
        "app/Cargo.toml",
        render(env, "project/app/Cargo.toml", &app)?,
    );
    plan.add_managed(
        "app/src/registry.rs",
        render(env, "project/app/registry.rs", &app)?,
    );
    plan.add_managed(
        "app/src/migrator.rs",
        render(env, "project/app/migrator.rs", &app)?,
    );

    plan.apply(ui, g.force, g.dry_run)?;

    if g.dry_run {
        return Ok(());
    }

    if !g.no_manifest {
        manifest.save(&root)?;
    } else {
        ui.warn("--no-manifest : Afrivel.toml non modifié (le projet peut diverger)");
    }

    rustfmt(&root, &snake, spec.is_some(), ui);

    ui.info(format!("✓ Module `{pascal}` généré dans {base}/."));
    if spec.is_some() {
        ui.info("  Prochaine étape : afrivel migrate");
    }
    Ok(())
}

/// Formate les fichiers générés (best-effort : `warn` si `rustfmt` est absent).
fn rustfmt(root: &Path, snake: &str, has_model: bool, ui: &Ui) {
    let dir = root.join("modules").join(snake).join("src");
    let mut files = vec![dir.join("lib.rs"), dir.join("routes.rs")];
    if has_model {
        for f in ["model.rs", "repository.rs", "controller.rs", "migration.rs"] {
            files.push(dir.join(f));
        }
    }
    files.push(root.join("app/src/registry.rs"));
    files.push(root.join("app/src/migrator.rs"));

    let ok = Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .args(&files)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if ok {
        ui.detail("rustfmt appliqué");
    } else {
        ui.warn("rustfmt indisponible : fichiers laissés tels quels (continue)");
    }
}
