//! Dépôt utilisateur **SeaORM** : implémentation de production du port [`UserRepository`].
//!
//! Résout les rôles via le pivot `user_roles`, puis les permissions via `role_permissions`
//! (RBAC : un utilisateur hérite des permissions de ses rôles).

use afrivel::Result;
use afrivel::orm::db_error;
use afrivel::orm::sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use async_trait::async_trait;
use chrono::Utc;

use crate::auth::contracts::{Credentials, UserRef, UserRepository};
use crate::auth::domain::entities::{permission, role, role_permission, user, user_role};

/// Dépôt utilisateur adossé à une connexion SeaORM.
#[derive(Clone)]
pub struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRepository {
    /// Construit le dépôt sur une connexion existante.
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    /// Noms des rôles correspondant à `ids`.
    async fn role_names(&self, ids: &[i64]) -> Result<Vec<String>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        Ok(role::Entity::find()
            .filter(role::Column::Id.is_in(ids.to_vec()))
            .all(&self.db)
            .await
            .map_err(db_error)?
            .into_iter()
            .map(|r| r.name)
            .collect())
    }

    /// Noms des permissions correspondant à `ids`.
    async fn permission_names(&self, ids: &[i64]) -> Result<Vec<String>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        Ok(permission::Entity::find()
            .filter(permission::Column::Id.is_in(ids.to_vec()))
            .all(&self.db)
            .await
            .map_err(db_error)?
            .into_iter()
            .map(|p| p.name)
            .collect())
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn create(&self, email: &str, password_hash: &str) -> Result<UserRef> {
        let now = Utc::now();
        let model = user::ActiveModel {
            email: Set(email.to_string()),
            password_hash: Set(password_hash.to_string()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        };
        let saved = model.insert(&self.db).await.map_err(db_error)?;
        Ok(UserRef {
            id: saved.id.to_string(),
            email: saved.email,
        })
    }

    async fn by_email(&self, email: &str) -> Result<Option<Credentials>> {
        let Some(found) = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&self.db)
            .await
            .map_err(db_error)?
        else {
            return Ok(None);
        };

        let role_ids: Vec<i64> = user_role::Entity::find()
            .filter(user_role::Column::UserId.eq(found.id))
            .all(&self.db)
            .await
            .map_err(db_error)?
            .into_iter()
            .map(|ur| ur.role_id)
            .collect();

        let permission_ids: Vec<i64> = if role_ids.is_empty() {
            Vec::new()
        } else {
            role_permission::Entity::find()
                .filter(role_permission::Column::RoleId.is_in(role_ids.clone()))
                .all(&self.db)
                .await
                .map_err(db_error)?
                .into_iter()
                .map(|rp| rp.permission_id)
                .collect()
        };

        Ok(Some(Credentials {
            id: found.id.to_string(),
            email: found.email,
            password_hash: found.password_hash,
            roles: self.role_names(&role_ids).await?,
            permissions: self.permission_names(&permission_ids).await?,
        }))
    }
}
