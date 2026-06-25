//! Test d'intégration Postgres (critère de sortie M2).
//!
//! Couvre : migrations `users` + `roles` + pivot `user_roles` (agrégées **dans le désordre**
//! puis triées par timestamp via [`afrivel_orm::migrator::sorted`]), CRUD via
//! [`afrivel_orm::repository`], insertion de relation N-N, puis rollback (`Migrator::down`).
//!
//! Ignoré silencieusement si `DATABASE_URL` est absent (exécuté en CI avec le service
//! Postgres ; voir `.github/workflows/ci.yml`).

use sea_orm::{ActiveValue::Set, Database, DatabaseConnection, EntityTrait};
use sea_orm_migration::prelude::*;

// --- Entités minimales -------------------------------------------------------

mod user {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i64,
        #[sea_orm(unique)]
        pub email: String,
        pub name: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

mod role {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "roles")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i64,
        pub name: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

mod user_role {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "user_roles")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub user_id: i64,
        #[sea_orm(primary_key, auto_increment = false)]
        pub role_id: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

// --- Identifiants de schéma --------------------------------------------------

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
    Name,
}

#[derive(DeriveIden)]
enum Roles {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum UserRoles {
    Table,
    UserId,
    RoleId,
}

// --- Migrations (timestamps volontairement déclarés dans le désordre) --------

struct CreateUsers;
impl MigrationName for CreateUsers {
    fn name(&self) -> &str {
        "m20240101_000001_create_users"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for CreateUsers {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

struct CreateRoles;
impl MigrationName for CreateRoles {
    fn name(&self) -> &str {
        "m20240101_000002_create_roles"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for CreateRoles {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Roles::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Roles::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Roles::Name).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Roles::Table).to_owned())
            .await
    }
}

struct CreatePivot;
impl MigrationName for CreatePivot {
    fn name(&self) -> &str {
        "m20240101_000003_create_user_roles"
    }
}
#[async_trait::async_trait]
impl MigrationTrait for CreatePivot {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRoles::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserRoles::UserId).big_integer().not_null())
                    .col(ColumnDef::new(UserRoles::RoleId).big_integer().not_null())
                    .primary_key(
                        Index::create()
                            .col(UserRoles::UserId)
                            .col(UserRoles::RoleId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRoles::Table, UserRoles::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(UserRoles::Table, UserRoles::RoleId)
                            .to(Roles::Table, Roles::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRoles::Table).to_owned())
            .await
    }
}

struct Migrator;
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        // Désordre intentionnel : `sorted` rétablit l'ordre chronologique par timestamp.
        afrivel_orm::migrator::sorted(vec![
            Box::new(CreatePivot),
            Box::new(CreateUsers),
            Box::new(CreateRoles),
        ])
    }
}

// --- Test --------------------------------------------------------------------

#[tokio::test]
async fn migrate_crud_relation_rollback() {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        eprintln!("DATABASE_URL absent — test Postgres ignoré");
        return;
    };
    let db: DatabaseConnection = Database::connect(url).await.expect("connexion Postgres");

    // Repart d'un schéma propre puis applique toutes les migrations (ordre rétabli).
    Migrator::fresh(&db).await.expect("migrations up");

    // CREATE
    let created = afrivel_orm::repository::create(
        &db,
        user::ActiveModel {
            email: Set("ada@example.com".to_owned()),
            name: Set("Ada".to_owned()),
            ..Default::default()
        },
    )
    .await
    .expect("create user");
    assert_eq!(created.email, "ada@example.com");

    let role = afrivel_orm::repository::create(
        &db,
        role::ActiveModel {
            name: Set("admin".to_owned()),
            ..Default::default()
        },
    )
    .await
    .expect("create role");

    // FIND
    let found = afrivel_orm::repository::find::<user::Entity, _>(&db, created.id)
        .await
        .expect("find user");
    assert_eq!(found.expect("user présent").name, "Ada");

    // RELATION N-N (insertion dans le pivot)
    afrivel_orm::repository::create(
        &db,
        user_role::ActiveModel {
            user_id: Set(created.id),
            role_id: Set(role.id),
        },
    )
    .await
    .expect("attach role");
    let links = user_role::Entity::find()
        .all(&db)
        .await
        .expect("read pivot");
    assert_eq!(links.len(), 1);

    // DELETE
    let affected = afrivel_orm::repository::delete::<user::Entity, _>(&db, created.id)
        .await
        .expect("delete user");
    assert_eq!(affected, 1);

    // ROLLBACK complet
    Migrator::down(&db, None).await.expect("migrations down");
}
