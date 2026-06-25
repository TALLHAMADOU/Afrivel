//! Vérifie que `#[derive(afrivel::Model)]` branche un `Model` SeaORM sur le trait
//! ergonomique `afrivel::orm::Model` (nom de table + CRUD par défaut).

use afrivel::orm::Model as ModelExt;

mod post {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, afrivel::Model)]
    #[sea_orm(table_name = "posts")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i64,
        pub title: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[test]
fn derive_exposes_table_name() {
    assert_eq!(<post::Model as ModelExt>::TABLE, "posts");
    assert_eq!(post::Model::table_name(), "posts");
}

// La présence des helpers CRUD par défaut (find/all/delete) est vérifiée à la compilation :
// ce module ne compile que si l'impl du trait est correctement générée.
#[allow(dead_code)]
fn crud_helpers_exist() {
    async fn _check(db: &sea_orm::DatabaseConnection) {
        let _ = post::Model::all(db).await;
        let _ = post::Model::find(db, 1).await;
        let _ = post::Model::delete(db, 1).await;
    }
}
