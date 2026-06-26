//! Garde-fou central (DR-014) : `new` → `make:module` → **`cargo build`** en tmpdir.
//!
//! Vérifie que la sortie de la CLI compile réellement contre les crates du framework
//! (référencées par chemin via `AFRIVEL_DEV_PATH`). C'est le test qui garantit que les
//! templates restent cohérents avec l'API d'Afrivel.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Dossier `crates/` de ce dépôt (parent du manifeste d'`afrivel-cli`).
fn crates_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crates/")
        .to_path_buf()
}

fn afrivel(cwd: &Path, dev_path: &Path, args: &[&str]) {
    let status = Command::new(env!("CARGO_BIN_EXE_afrivel"))
        .args(args)
        .current_dir(cwd)
        .env("AFRIVEL_DEV_PATH", dev_path)
        .status()
        .expect("exécute afrivel");
    assert!(status.success(), "afrivel {args:?} a échoué");
}

#[test]
fn generated_project_compiles() {
    let crates = crates_dir();
    let tmp = tempfile::tempdir().expect("tempdir");

    // 1. Scaffold du projet.
    afrivel(tmp.path(), &crates, &["new", "demo"]);
    let project = tmp.path().join("demo");
    assert!(project.join("Afrivel.toml").is_file());

    // 2. Module avec modèle (CRUD + migration) couvrant plusieurs types/modificateurs.
    afrivel(
        &project,
        &crates,
        &[
            "make:module",
            "Post",
            "--model",
            "title:string,body:text:nullable,price:decimal:default=0,slug:string:unique,code:uuid",
        ],
    );
    // 3. Module sans modèle (squelette compilable).
    afrivel(&project, &crates, &["make:module", "Health"]);

    // 4. Garde-fou : le workspace généré compile.
    let status = Command::new(env!("CARGO"))
        .args(["build"])
        .current_dir(&project)
        .status()
        .expect("cargo build");
    assert!(status.success(), "le projet généré ne compile pas");
}
