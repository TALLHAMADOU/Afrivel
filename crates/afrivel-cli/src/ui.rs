//! Sorties console et type d'erreur de la CLI.
//!
//! Format des messages : `✗ <quoi> — <pourquoi> → <comment corriger>` (cf. docs/CLI.md).
//! Codes de sortie : `0` ok, `1` erreur utilisateur, `2` erreur interne.

use std::fmt;

use crate::cli::Globals;

/// Erreur remontée par une commande, porteuse de son code de sortie.
#[derive(Debug)]
pub struct CliError {
    pub message: String,
    pub code: i32,
}

impl CliError {
    /// Erreur utilisateur (mauvaise saisie, état du projet) — code 1.
    pub fn user(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: 1,
        }
    }

    /// Erreur interne (I/O, rendu, sous-processus) — code 2.
    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            code: 2,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        Self::internal(format!("I/O — {e}"))
    }
}

impl From<afrivel_codegen::ParseError> for CliError {
    fn from(e: afrivel_codegen::ParseError) -> Self {
        Self::user(format!(
            "--model invalide — {e} → voir `afrivel make:module --help`"
        ))
    }
}

pub type CliResult = Result<(), CliError>;

/// Verbosité et rendu des messages, dérivés des flags globaux.
#[derive(Debug, Clone, Copy)]
pub struct Ui {
    quiet: bool,
    verbose: bool,
}

impl Ui {
    pub fn new(g: &Globals) -> Self {
        Self {
            quiet: g.quiet,
            verbose: g.verbose,
        }
    }

    /// Message de succès/étape (masqué en `--quiet`).
    pub fn info(&self, msg: impl AsRef<str>) {
        if !self.quiet {
            println!("{}", msg.as_ref());
        }
    }

    /// Détail (uniquement en `--verbose`).
    pub fn detail(&self, msg: impl AsRef<str>) {
        if self.verbose && !self.quiet {
            println!("  {}", msg.as_ref());
        }
    }

    /// Avertissement non bloquant (toujours affiché sur stderr).
    pub fn warn(&self, msg: impl AsRef<str>) {
        eprintln!("⚠ {}", msg.as_ref());
    }
}
