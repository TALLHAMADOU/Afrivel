//! Fabriques de données de test/seed (inspirées des factories Eloquent).

use afrivel_core::Result;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, EntityTrait, IntoActiveModel,
};

use crate::repository;

/// Décrit comment fabriquer des instances d'une entité.
///
/// L'implémenteur fournit `definition()` (un modèle actif gabarit, éventuellement
/// randomisé) ; les helpers `create`/`create_many` persistent les instances.
pub trait Factory {
    /// Modèle actif produit par la fabrique.
    type ActiveModel: ActiveModelTrait + ActiveModelBehavior + Send;

    /// Construit un modèle actif non persistant.
    fn definition() -> Self::ActiveModel;

    /// Persiste une instance et renvoie l'enregistrement créé.
    fn create<C>(
        db: &C,
    ) -> impl std::future::Future<
        Output = Result<<<Self::ActiveModel as ActiveModelTrait>::Entity as EntityTrait>::Model>,
    > + Send
    where
        C: ConnectionTrait,
        <<Self::ActiveModel as ActiveModelTrait>::Entity as EntityTrait>::Model:
            IntoActiveModel<Self::ActiveModel>,
    {
        async move { repository::create(db, Self::definition()).await }
    }

    /// Persiste `count` instances.
    fn create_many<C>(
        db: &C,
        count: usize,
    ) -> impl std::future::Future<
        Output = Result<
            Vec<<<Self::ActiveModel as ActiveModelTrait>::Entity as EntityTrait>::Model>,
        >,
    > + Send
    where
        C: ConnectionTrait,
        <<Self::ActiveModel as ActiveModelTrait>::Entity as EntityTrait>::Model:
            IntoActiveModel<Self::ActiveModel>,
    {
        async move {
            let mut out = Vec::with_capacity(count);
            for _ in 0..count {
                out.push(repository::create(db, Self::definition()).await?);
            }
            Ok(out)
        }
    }
}
