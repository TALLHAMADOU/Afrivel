//! Agrège les migrations de tous les modules et les trie par timestamp (ordre déterministe).

use afrivel::orm::sea_orm_migration::prelude::*;

use crate::auth;

/// Migrator de l'application (consommé par les commandes `migrate*`).
pub struct Migrator;

impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        let mut all = Vec::new();
        all.extend(auth::migrations());
        afrivel::orm::migrator::sorted(all)
    }
}
