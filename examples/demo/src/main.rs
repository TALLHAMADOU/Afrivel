//! Binaire de l'app démo : dispatch des sous-commandes runtime partagées (`afrivel-cli-rt`).
//!
//! Reproduit le `main.rs` généré par la CLI ; sert le module Auth et applique ses migrations.

use afrivel::auth::jwt::JwtSecret;
use afrivel::orm::sea_orm::{Database, DatabaseConnection};
use afrivel::orm::sea_orm_migration::MigratorTrait;
use afrivel_cli_rt::RuntimeCommand;
use clap::Parser;
use demo::migrator::Migrator;
use demo::registry;

type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Parser)]
#[command(name = "demo", about = "Démo Afrivel — runtime du module Auth")]
struct Cli {
    #[command(subcommand)]
    command: RuntimeCommand,
}

#[afrivel::tokio::main(crate = "afrivel::tokio")]
async fn main() -> Result<(), BoxError> {
    afrivel::logging::init();

    match Cli::parse().command {
        RuntimeCommand::Serve { port, host } => {
            let db = connect().await?;
            let addr: std::net::SocketAddr = format!("{host}:{port}").parse()?;
            registry::application(db, secret()).serve(addr).await?;
        }
        RuntimeCommand::Migrate => {
            Migrator::up(&connect().await?, None).await?;
        }
        RuntimeCommand::MigrateRollback { steps } => {
            Migrator::down(&connect().await?, Some(steps)).await?;
        }
        RuntimeCommand::MigrateFresh => {
            Migrator::fresh(&connect().await?).await?;
        }
        RuntimeCommand::MigrateStatus => {
            Migrator::status(&connect().await?).await?;
        }
        RuntimeCommand::DbSeed => {
            println!("Aucun seeder enregistré.");
        }
        RuntimeCommand::RouteList => {
            for module in registry::application(connect().await?, secret()).modules() {
                println!("module: {module}");
            }
        }
    }
    Ok(())
}

/// Ouvre une connexion à la base depuis `DATABASE_URL`.
async fn connect() -> Result<DatabaseConnection, BoxError> {
    let url =
        std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL n'est pas défini (voir .env)")?;
    Ok(Database::connect(url).await?)
}

/// Secret de signature JWT, depuis `APP_KEY` (valeur de dev par défaut sinon).
fn secret() -> JwtSecret {
    let key = std::env::var("APP_KEY").unwrap_or_else(|_| "dev-insecure-key-change-me".into());
    JwtSecret::new(key)
}
