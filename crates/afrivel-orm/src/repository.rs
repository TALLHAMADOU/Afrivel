//! Helpers CRUD ergonomiques au-dessus de SeaORM.
//!
//! Fonctions génériques qui enveloppent les opérations SeaORM et renvoient le [`Result`]
//! unifié d'Afrivel (erreurs SGBD traduites via [`db_error`]). Pensées pour être appelées
//! depuis les services des modules générés.

use afrivel_core::Result;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, EntityTrait, IntoActiveModel,
    PrimaryKeyTrait,
};

use crate::error::db_error;

/// Insère un modèle actif et renvoie l'enregistrement persistant (via `RETURNING`).
pub async fn create<A, C>(db: &C, model: A) -> Result<<A::Entity as EntityTrait>::Model>
where
    C: ConnectionTrait,
    A: ActiveModelTrait + ActiveModelBehavior + Send,
    <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
{
    model.insert(db).await.map_err(db_error)
}

/// Met à jour un modèle actif et renvoie l'enregistrement à jour.
pub async fn update<A, C>(db: &C, model: A) -> Result<<A::Entity as EntityTrait>::Model>
where
    C: ConnectionTrait,
    A: ActiveModelTrait + ActiveModelBehavior + Send,
    <A::Entity as EntityTrait>::Model: IntoActiveModel<A>,
{
    model.update(db).await.map_err(db_error)
}

/// Recherche un enregistrement par clé primaire.
pub async fn find<E, C>(
    db: &C,
    id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
) -> Result<Option<E::Model>>
where
    E: EntityTrait,
    C: ConnectionTrait,
{
    E::find_by_id(id).one(db).await.map_err(db_error)
}

/// Recherche un enregistrement par clé primaire ou renvoie [`afrivel_core::Error::NotFound`].
pub async fn find_or_fail<E, C>(
    db: &C,
    id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType,
) -> Result<E::Model>
where
    E: EntityTrait,
    C: ConnectionTrait,
{
    find::<E, C>(db, id)
        .await?
        .ok_or(afrivel_core::Error::NotFound)
}

/// Renvoie tous les enregistrements de l'entité.
pub async fn all<E, C>(db: &C) -> Result<Vec<E::Model>>
where
    E: EntityTrait,
    C: ConnectionTrait,
{
    E::find().all(db).await.map_err(db_error)
}

/// Supprime un enregistrement par clé primaire ; renvoie le nombre de lignes affectées.
pub async fn delete<E, C>(db: &C, id: <E::PrimaryKey as PrimaryKeyTrait>::ValueType) -> Result<u64>
where
    E: EntityTrait,
    C: ConnectionTrait,
{
    let res = E::delete_by_id(id).exec(db).await.map_err(db_error)?;
    Ok(res.rows_affected)
}
