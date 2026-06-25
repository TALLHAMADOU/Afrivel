//! Couche ORM ergonomique d'Afrivel au-dessus de SeaORM/sqlx.
//!
//! Fournit des helpers CRUD ([`repository`]), des [`Factory`]/[`Seeder`] inspirés
//! d'Eloquent, et l'agrégation/ordonnancement des migrations par timestamp
//! ([`migrator`], DR-021). Les opérations renvoient le [`afrivel_core::Result`] unifié
//! (erreurs SGBD traduites par [`error::db_error`]).
//!
//! Re-exporte `sea_orm` et `sea_orm_migration` pour que les modules générés n'aient qu'à
//! dépendre de `afrivel`.

#![forbid(unsafe_code)]

pub mod error;
pub mod factory;
pub mod migrator;
pub mod model;
pub mod repository;
pub mod seeder;

pub use error::db_error;
pub use factory::Factory;
pub use model::Model;
pub use seeder::{Seeder, run_all as run_seeders};

/// Re-export complet de SeaORM (entités, `ActiveModel`, connexion, requêtes).
pub use sea_orm;
/// Re-export de l'outillage de migration SeaORM.
pub use sea_orm_migration;
