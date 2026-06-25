//! Façade du framework Afrivel.
//!
//! Re-exporte les API publiques des crates internes ; c'est la dépendance que les
//! applications et modules générés importent. Le noyau HTTP est re-exporté à la racine ;
//! la couche persistance est disponible sous [`orm`], et `#[derive(Model)]` est re-exporté
//! depuis `afrivel-macros`.

pub use afrivel_core::*;

/// Couche ORM (SeaORM ergonomique : CRUD, relations, factories, seeders, migrator).
pub use afrivel_orm as orm;

/// `#[derive(Model)]` — branche un `Model` SeaORM sur le trait [`orm::Model`].
pub use afrivel_macros::Model;

/// Re-exports les plus courants pour les applications et modules générés.
pub mod prelude {
    pub use afrivel_core::prelude::*;
    pub use afrivel_macros::Model;
    pub use afrivel_orm::{Factory, Model, Seeder};
}
