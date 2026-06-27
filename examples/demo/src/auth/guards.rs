//! Gardes RBAC du module Auth, utilisées via l'extracteur `Authorized<G>` du noyau.

use afrivel::Result;
use afrivel::auth::{AuthUser, Guard};

/// Exige le rôle `admin`.
pub struct AdminGuard;

impl Guard for AdminGuard {
    fn authorize(user: &AuthUser) -> Result<()> {
        user.require_role("admin")
    }
}
