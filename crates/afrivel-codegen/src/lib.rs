//! Logique de génération partagée d'Afrivel.
//!
//! Source **unique** pour le parsing du DSL `--model`, le mapping de types et les règles de
//! nommage, réutilisée par `afrivel-cli` (génération de fichiers) et `afrivel-macros`
//! (`#[derive(Model)]`). N'a **aucune dépendance runtime** : uniquement de la logique pure,
//! testée unitairement, garantissant la cohérence entre les deux générateurs.

#![forbid(unsafe_code)]

pub mod error;
pub mod model;
pub mod naming;
pub mod types;

pub use error::ParseError;
pub use model::{Field, ModelSpec, Modifiers};
pub use naming::{camel_case, pascal_case, pluralize, snake_case, table_name};
pub use types::FieldType;
