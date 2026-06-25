//! Contrat des sous-commandes **runtime** d'Afrivel, partagé entre le binaire `afrivel`
//! (CLI globale) et le binaire `app` du projet généré.
//!
//! La CLI globale délègue ces commandes en lançant `cargo run -p app -- <cmd>` ; le binaire
//! `app` les exécute en montant l'`Application`. Définir l'énumération **ici, une seule
//! fois**, garantit à la compilation que les deux côtés s'accordent sur les noms et les
//! arguments (pas de couplage par chaînes magiques).

#![forbid(unsafe_code)]

use clap::Subcommand;

/// Version du contrat runtime. La CLI globale compare cette valeur à celle du projet pour
/// avertir en cas de dérive (CLI plus récente/ancienne que les crates `afrivel-*` du projet).
pub const RUNTIME_CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Sous-commandes exécutées par le runtime du projet (binaire `app`).
#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum RuntimeCommand {
    /// Lance le serveur HTTP.
    Serve {
        /// Port d'écoute.
        #[arg(long, default_value_t = 3000)]
        port: u16,
        /// Adresse d'écoute.
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
    },
    /// Applique les migrations en attente (ordre par timestamp).
    Migrate,
    /// Annule les dernières migrations appliquées.
    #[command(name = "migrate:rollback")]
    MigrateRollback {
        /// Nombre d'étapes à annuler.
        #[arg(long, default_value_t = 1)]
        steps: u32,
    },
    /// Réinitialise la base puis ré-applique toutes les migrations.
    #[command(name = "migrate:fresh")]
    MigrateFresh,
    /// Affiche l'état des migrations (appliquées / en attente).
    #[command(name = "migrate:status")]
    MigrateStatus,
    /// Exécute les seeders.
    #[command(name = "db:seed")]
    DbSeed,
    /// Affiche la table des routes (connue du seul runtime).
    #[command(name = "route:list")]
    RouteList,
}

impl RuntimeCommand {
    /// Nom canonique de la commande (tel qu'attendu en CLI et en délégation).
    pub fn name(&self) -> &'static str {
        match self {
            RuntimeCommand::Serve { .. } => "serve",
            RuntimeCommand::Migrate => "migrate",
            RuntimeCommand::MigrateRollback { .. } => "migrate:rollback",
            RuntimeCommand::MigrateFresh => "migrate:fresh",
            RuntimeCommand::MigrateStatus => "migrate:status",
            RuntimeCommand::DbSeed => "db:seed",
            RuntimeCommand::RouteList => "route:list",
        }
    }

    /// Indique si un nom de commande relève du runtime (à déléguer au binaire `app`).
    pub fn is_runtime(name: &str) -> bool {
        matches!(
            name,
            "serve"
                | "migrate"
                | "migrate:rollback"
                | "migrate:fresh"
                | "migrate:status"
                | "db:seed"
                | "route:list"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[derive(Parser)]
    struct Harness {
        #[command(subcommand)]
        cmd: RuntimeCommand,
    }

    #[test]
    fn parses_serve_with_defaults() {
        let h = Harness::try_parse_from(["app", "serve"]).unwrap();
        assert_eq!(
            h.cmd,
            RuntimeCommand::Serve {
                port: 3000,
                host: "127.0.0.1".to_string()
            }
        );
        assert_eq!(h.cmd.name(), "serve");
    }

    #[test]
    fn parses_namespaced_commands() {
        let h = Harness::try_parse_from(["app", "migrate:fresh"]).unwrap();
        assert_eq!(h.cmd, RuntimeCommand::MigrateFresh);
        let h = Harness::try_parse_from(["app", "db:seed"]).unwrap();
        assert_eq!(h.cmd.name(), "db:seed");
    }

    #[test]
    fn runtime_classification() {
        assert!(RuntimeCommand::is_runtime("route:list"));
        assert!(RuntimeCommand::is_runtime("migrate"));
        assert!(!RuntimeCommand::is_runtime("make:module"));
        assert!(!RuntimeCommand::is_runtime("new"));
    }
}
