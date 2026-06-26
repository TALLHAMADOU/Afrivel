//! Commandes hors-génération : délégation runtime, `module:list`, `completion`.

use std::io;
use std::process::Command;

use afrivel_cli_rt::RuntimeCommand;
use clap::CommandFactory;

use crate::cli::Cli;
use crate::manifest::Manifest;
use crate::ui::{CliError, CliResult, Ui};

/// Délègue une commande runtime au binaire `app` (`cargo run -p app -- <cmd>`).
pub fn delegate(args: &[String]) -> CliResult {
    let name = args.first().map(String::as_str).unwrap_or("");
    if !RuntimeCommand::is_runtime(name) {
        return Err(CliError::user(format!(
            "✗ Commande inconnue : `{name}` → voir `afrivel --help`"
        )));
    }

    let cwd = std::env::current_dir()?;
    let root = Manifest::find_root(&cwd).ok_or_else(|| {
        CliError::user("✗ Pas un projet Afrivel (Afrivel.toml absent) → lance `afrivel new`")
    })?;

    let status = Command::new("cargo")
        .arg("run")
        .arg("-q")
        .arg("-p")
        .arg("app")
        .arg("--")
        .args(args)
        .current_dir(&root)
        .status()
        .map_err(|e| CliError::internal(format!("lancement de `cargo run -p app` — {e}")))?;

    if status.success() {
        Ok(())
    } else {
        // La sortie du sous-processus a déjà été affichée ; on propage le code.
        Err(CliError {
            message: String::new(),
            code: status.code().unwrap_or(2),
        })
    }
}

/// `module:list` — lit le manifeste (hors-ligne).
pub fn module_list(ui: &Ui) -> CliResult {
    let cwd = std::env::current_dir()?;
    let root = Manifest::find_root(&cwd).ok_or_else(|| {
        CliError::user("✗ Pas un projet Afrivel (Afrivel.toml absent) → lance `afrivel new`")
    })?;
    let manifest = Manifest::load(&root)?;
    if manifest.modules.is_empty() {
        ui.info("Aucun module enregistré.");
    } else {
        ui.info(format!("Modules ({}) :", manifest.modules.len()));
        for (name, entry) in &manifest.modules {
            match &entry.model {
                Some(m) => ui.info(format!("  • {name}  (model: {m})")),
                None => ui.info(format!("  • {name}")),
            }
        }
    }
    Ok(())
}

/// `completion <shell>` — émet le script de complétion sur stdout.
pub fn completion(shell: clap_complete::Shell) -> CliResult {
    let mut cmd = Cli::command();
    clap_complete::generate(shell, &mut cmd, "afrivel", &mut io::stdout());
    Ok(())
}
