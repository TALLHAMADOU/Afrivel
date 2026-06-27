//! Migration du module Auth : crée `users`, `roles`, `permissions` et les pivots.
//!
//! Le préfixe `m0000000001_` ordonne cette migration en tête (cf. tri par timestamp du
//! `migrator`) : `users` doit exister avant tout module portant une FK `user_id`.

use afrivel::orm::sea_orm::Schema;
use afrivel::orm::sea_orm_migration::prelude::*;

use crate::auth::domain::entities::{permission, role, role_permission, user, user_role};

/// Migration de création du schéma Auth.
pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m0000000001_create_auth_tables"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let schema = Schema::new(manager.get_database_backend());
        manager
            .create_table(schema.create_table_from_entity(user::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(role::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(permission::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(user_role::Entity))
            .await?;
        manager
            .create_table(schema.create_table_from_entity(role_permission::Entity))
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for table in [
            Table::drop().table(role_permission::Entity).to_owned(),
            Table::drop().table(user_role::Entity).to_owned(),
            Table::drop().table(permission::Entity).to_owned(),
            Table::drop().table(role::Entity).to_owned(),
            Table::drop().table(user::Entity).to_owned(),
        ] {
            manager.drop_table(table).await?;
        }
        Ok(())
    }
}
