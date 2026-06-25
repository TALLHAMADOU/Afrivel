//! Seeders : peuplement initial de la base, orchestrables en liste.

use afrivel_core::Result;
use async_trait::async_trait;
use sea_orm::DatabaseConnection;

/// Un seeder peuple la base. `async_trait` + connexion concrète pour rester
/// dyn-compatible (les seeders sont exécutés via une liste `Vec<Box<dyn Seeder>>`).
#[async_trait]
pub trait Seeder: Send + Sync {
    /// Nom lisible (logs / sélection ciblée).
    fn name(&self) -> &str;

    /// Exécute le peuplement.
    async fn run(&self, db: &DatabaseConnection) -> Result<()>;
}

/// Exécute une liste de seeders dans l'ordre fourni.
pub async fn run_all(db: &DatabaseConnection, seeders: &[Box<dyn Seeder>]) -> Result<()> {
    for seeder in seeders {
        seeder.run(db).await?;
    }
    Ok(())
}
