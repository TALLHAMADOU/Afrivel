//! Application de démonstration Afrivel : prouve le flux **inscription → connexion →
//! accès protégé** de bout en bout via le module [`auth`] (Clean Architecture).
//!
//! Exposée comme bibliothèque pour que les tests d'intégration (`tests/`) montent le module
//! avec un dépôt en mémoire, sans base de données.

pub mod auth;
pub mod migrator;
pub mod registry;
