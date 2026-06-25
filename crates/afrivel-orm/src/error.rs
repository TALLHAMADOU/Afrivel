//! Pont entre les erreurs SeaORM et le type d'erreur unifié d'Afrivel.

use afrivel_core::Error;
use sea_orm::DbErr;

/// Convertit une erreur SeaORM en [`afrivel_core::Error`].
///
/// `RecordNotFound` est mappé sur [`Error::NotFound`] (404) ; tout le reste sur
/// [`Error::Database`], dont les détails sont masqués au client par l'`IntoResponse` du
/// noyau.
pub fn db_error(err: DbErr) -> Error {
    match err {
        DbErr::RecordNotFound(_) => Error::NotFound,
        other => Error::Database(other.to_string()),
    }
}
