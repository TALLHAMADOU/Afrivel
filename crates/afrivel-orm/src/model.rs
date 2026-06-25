//! Trait `Model` : enrichit un `Model` SeaORM d'un CRUD ergonomique.
//!
//! Implémenté par `#[derive(afrivel::Model)]` (crate `afrivel-macros`). Les méthodes par
//! défaut délèguent à [`crate::repository`], si bien qu'un modèle généré dispose
//! directement de `User::find(db, id)`, `User::all(db)`, etc.

use afrivel_core::Result;
use sea_orm::{ConnectionTrait, EntityTrait, FromQueryResult, ModelTrait, PrimaryKeyTrait};

use crate::repository;

/// Entité SeaORM associée au modèle `M`.
pub type EntityOf<M> = <M as ModelTrait>::Entity;

/// Type de la valeur de clé primaire de l'entité associée à `M`.
pub type PrimaryKeyValue<M> =
    <<EntityOf<M> as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType;

/// Glue ergonomique au-dessus d'un `Model` SeaORM.
///
/// Fournit le nom de table et des helpers de lecture/suppression branchés sur l'`Entity`
/// (réutilise l'`Entity` de [`ModelTrait`]). La création/mise à jour passe par
/// [`repository::create`]/[`repository::update`] (contraintes de bornes sur l'`ActiveModel`).
pub trait Model: ModelTrait + FromQueryResult + Sized
where
    EntityOf<Self>: EntityTrait<Model = Self>,
{
    /// Nom de la table (miroir de l'attribut `#[sea_orm(table_name = …)]`).
    const TABLE: &'static str;

    /// Nom de table (forme méthode, pratique en contexte dynamique).
    fn table_name() -> &'static str {
        Self::TABLE
    }

    /// Recherche par clé primaire.
    fn find<C>(
        db: &C,
        id: PrimaryKeyValue<Self>,
    ) -> impl std::future::Future<Output = Result<Option<Self>>> + Send
    where
        C: ConnectionTrait,
    {
        repository::find::<EntityOf<Self>, C>(db, id)
    }

    /// Recherche par clé primaire ou échec [`afrivel_core::Error::NotFound`].
    fn find_or_fail<C>(
        db: &C,
        id: PrimaryKeyValue<Self>,
    ) -> impl std::future::Future<Output = Result<Self>> + Send
    where
        C: ConnectionTrait,
    {
        repository::find_or_fail::<EntityOf<Self>, C>(db, id)
    }

    /// Tous les enregistrements.
    fn all<C>(db: &C) -> impl std::future::Future<Output = Result<Vec<Self>>> + Send
    where
        C: ConnectionTrait,
    {
        repository::all::<EntityOf<Self>, C>(db)
    }

    /// Supprime par clé primaire ; renvoie le nombre de lignes affectées.
    fn delete<C>(
        db: &C,
        id: PrimaryKeyValue<Self>,
    ) -> impl std::future::Future<Output = Result<u64>> + Send
    where
        C: ConnectionTrait,
    {
        repository::delete::<EntityOf<Self>, C>(db, id)
    }
}
