//! Façade du framework Afrivel.
//!
//! Re-exporte les API publiques des crates internes ; c'est la dépendance que les applications
//! générées importent. En M1, seule la couche noyau (`afrivel-core`) est disponible ;
//! l'ORM et les macros seront re-exportés en M2.

pub use afrivel_core::*;

/// Re-exports les plus courants pour les applications et modules générés.
pub mod prelude {
    pub use afrivel_core::prelude::*;
}
