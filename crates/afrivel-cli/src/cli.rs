//! Définition clap de la CLI `afrivel` : flags globaux et sous-commandes.
//!
//! Les commandes **runtime** (`serve`, `migrate*`, `db:seed`, `route:list`) ne sont pas
//! déclarées ici : elles sont capturées par [`Command::Runtime`] (`external_subcommand`)
//! puis déléguées au binaire `app` du projet. Le contrat [`afrivel_cli_rt`] reste l'autorité
//! sur leurs noms (via `RuntimeCommand::is_runtime`).

use clap::{Args, Parser, Subcommand};

/// CLI globale du framework Afrivel.
#[derive(Debug, Parser)]
#[command(
    name = "afrivel",
    version,
    about = "Afrivel — le confort Laravel, la rigueur Rust."
)]
pub struct Cli {
    #[command(flatten)]
    pub globals: Globals,

    #[command(subcommand)]
    pub command: Command,
}

/// Flags transverses, valables avant ou après la sous-commande.
#[derive(Debug, Clone, Default, Args)]
pub struct Globals {
    /// Silencieux : n'émet que les erreurs.
    #[arg(long, global = true)]
    pub quiet: bool,
    /// Verbeux : détaille les actions.
    #[arg(long, global = true)]
    pub verbose: bool,
    /// Écrase les fichiers existants en cas de collision.
    #[arg(long, global = true)]
    pub force: bool,
    /// Affiche le plan d'écriture sans rien écrire.
    #[arg(long, global = true)]
    pub dry_run: bool,
    /// N'écrit pas dans `Afrivel.toml`.
    #[arg(long, global = true)]
    pub no_manifest: bool,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Crée un nouveau projet Afrivel (workspace complet + git init).
    New {
        /// Nom du projet (dossier créé).
        name: String,
    },

    /// Génère un module complet (+ CRUD de lecture si `--model`).
    #[command(name = "make:module")]
    MakeModule {
        /// Nom du module (PascalCase).
        name: String,
        /// Spécification du modèle : `Champ:type[:modificateur][,…]`.
        #[arg(long)]
        model: Option<String>,
    },

    /// Liste les modules enregistrés (lecture de `Afrivel.toml`).
    #[command(name = "module:list")]
    ModuleList,

    /// Watch + rebuild + restart du serveur de développement.
    Dev {
        /// Port d'écoute transmis à `serve`.
        #[arg(long, default_value_t = 3000)]
        port: u16,
    },

    /// Génère un script de complétion shell.
    Completion {
        /// Shell cible.
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },

    /// Commandes runtime déléguées au binaire `app` (`serve`, `migrate*`, `db:seed`, …).
    #[command(external_subcommand)]
    Runtime(Vec<String>),
}
