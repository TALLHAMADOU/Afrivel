//! `afrivel new <nom>` — scaffolde un workspace de projet complet.

use std::path::Path;
use std::process::Command;

use afrivel_codegen::naming;

use crate::context::{AFRIVEL_VERSION, DepLines, ProjectContext};
use crate::manifest::Manifest;
use crate::templates::{Environment, render};
use crate::ui::{CliError, CliResult, Ui};
use crate::writer::Plan;

pub fn run(
    ui: &Ui,
    env: &Environment<'static>,
    name: &str,
    force: bool,
    dry_run: bool,
) -> CliResult {
    if !naming::is_valid_ident(name) {
        return Err(CliError::user(format!(
            "✗ Nom de projet invalide : `{name}` → identifiant Rust attendu (lettres, chiffres, `_`)"
        )));
    }

    let root = std::env::current_dir()?.join(name);
    if root.exists()
        && root
            .read_dir()
            .map(|mut d| d.next().is_some())
            .unwrap_or(false)
        && !force
    {
        return Err(CliError::user(format!(
            "✗ `{}` existe et n'est pas vide → choisis un autre nom ou utilise --force",
            root.display()
        )));
    }

    let deps = DepLines::resolve();
    let project = ProjectContext {
        name: name.to_string(),
        snake_name: naming::snake_case(name),
        afrivel_version: AFRIVEL_VERSION.to_string(),
    };
    // Projet vierge : aucun module encore enregistré.
    let manifest = Manifest {
        project: crate::manifest::Project {
            name: name.to_string(),
            afrivel_version: AFRIVEL_VERSION.to_string(),
        },
        modules: Default::default(),
    };
    let app = manifest.app_context(&deps);

    let mut plan = Plan::new(&root);
    plan.add("Cargo.toml", render(env, "project/Cargo.toml", ())?);
    plan.add(
        "Afrivel.toml",
        render(env, "project/Afrivel.toml", &project)?,
    );
    plan.add(".gitignore", render(env, "project/gitignore", ())?);
    plan.add(
        "config/default.toml",
        render(env, "project/config/default.toml", &project)?,
    );
    plan.add(
        "app/Cargo.toml",
        render(env, "project/app/Cargo.toml", &app)?,
    );
    plan.add("app/src/main.rs", render(env, "project/app/main.rs", ())?);
    plan.add(
        "app/src/registry.rs",
        render(env, "project/app/registry.rs", &app)?,
    );
    plan.add(
        "app/src/migrator.rs",
        render(env, "project/app/migrator.rs", &app)?,
    );

    plan.apply(ui, force, dry_run)?;

    if dry_run {
        return Ok(());
    }

    git_init(&root, ui);

    ui.info(format!("✓ Projet `{name}` créé."));
    ui.info("  Prochaines étapes :");
    ui.info(format!("    cd {name}"));
    ui.info("    afrivel make:module Post --model title:string,body:text");
    ui.info("    afrivel migrate && afrivel serve");
    Ok(())
}

fn git_init(root: &Path, ui: &Ui) {
    match Command::new("git")
        .arg("init")
        .arg("-q")
        .current_dir(root)
        .status()
    {
        Ok(s) if s.success() => ui.detail("git init"),
        _ => ui.warn("git indisponible : dépôt non initialisé (continue)"),
    }
}
