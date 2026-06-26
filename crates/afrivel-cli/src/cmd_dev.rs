//! `afrivel dev` — watch + rebuild + restart du serveur de développement.
//!
//! Stratégie (docs/CLI.md) : sur changement, on rebuild ; si le build échoue, l'ancien
//! process reste **vivant** ; sinon on le remplace. Délègue le lancement au binaire `app`.

use std::path::Path;
use std::process::{Child, Command};
use std::sync::mpsc;
use std::time::Duration;

use notify::{RecursiveMode, Watcher};

use crate::manifest::Manifest;
use crate::ui::{CliError, CliResult, Ui};

pub fn run(ui: &Ui, port: u16) -> CliResult {
    let cwd = std::env::current_dir()?;
    let root = Manifest::find_root(&cwd).ok_or_else(|| {
        CliError::user("✗ Pas un projet Afrivel (Afrivel.toml absent) → lance `afrivel new`")
    })?;

    if !build(&root) {
        return Err(CliError::user(
            "✗ Build initial en échec — corrige les erreurs puis relance",
        ));
    }
    let mut child = serve(&root, port)?;
    ui.info(format!(
        "✓ running http://127.0.0.1:{port}  (Ctrl-C pour quitter)"
    ));

    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })
    .map_err(|e| CliError::internal(format!("watcher — {e}")))?;

    for rel in ["app/src", "config"] {
        let p = root.join(rel);
        if p.exists() {
            let _ = watcher.watch(&p, RecursiveMode::Recursive);
        }
    }
    let modules = root.join("modules");
    if modules.exists() {
        let _ = watcher.watch(&modules, RecursiveMode::Recursive);
    }

    while rx.recv().is_ok() {
        // Debounce : absorbe la rafale d'événements.
        while rx.recv_timeout(Duration::from_millis(300)).is_ok() {}
        ui.info("⟳ building…");
        if build(&root) {
            let _ = child.kill();
            let _ = child.wait();
            child = serve(&root, port)?;
            ui.info(format!("✓ running :{port}"));
        } else {
            ui.warn("build en échec — ancien process conservé");
        }
    }
    let _ = child.kill();
    Ok(())
}

fn build(root: &Path) -> bool {
    Command::new("cargo")
        .args(["build", "-p", "app"])
        .current_dir(root)
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn serve(root: &Path, port: u16) -> Result<Child, CliError> {
    Command::new("cargo")
        .args(["run", "-q", "-p", "app", "--", "serve", "--port"])
        .arg(port.to_string())
        .current_dir(root)
        .spawn()
        .map_err(|e| CliError::internal(format!("lancement de `serve` — {e}")))
}
