//! Hachage de mots de passe avec **Argon2** (id, paramètres par défaut sûrs).
//!
//! Le sel est généré aléatoirement et stocké dans la chaîne PHC renvoyée par
//! [`hash_password`] ; [`verify_password`] le relit — aucun sel à gérer côté appelant.

use argon2::Argon2;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};

use crate::error::{Error, Result};

/// Hache un mot de passe en clair et renvoie sa représentation **PHC** (sel inclus).
///
/// Renvoie [`Error::Internal`] si le hachage échoue (cas anormal, p. ex. RNG indisponible).
pub fn hash_password(plain: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(plain.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| Error::Internal(format!("argon2 hash: {e}")))
}

/// Vérifie un mot de passe en clair contre un hachage **PHC**.
///
/// Renvoie `true` si le mot de passe correspond. Un hachage invalide ou une non-correspondance
/// renvoient `false` (jamais d'erreur) : l'appelant traduit l'échec en [`Error::Unauthorized`].
pub fn verify_password(plain: &str, phc_hash: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(phc_hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(plain.as_bytes(), &parsed)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_then_verify_roundtrip() {
        let hash = hash_password("s3cret!").expect("hash");
        assert!(hash.starts_with("$argon2"));
        assert!(verify_password("s3cret!", &hash));
        assert!(!verify_password("wrong", &hash));
    }

    #[test]
    fn distinct_salts_yield_distinct_hashes() {
        let a = hash_password("same").expect("hash a");
        let b = hash_password("same").expect("hash b");
        assert_ne!(a, b, "le sel aléatoire doit différencier les hachages");
        assert!(verify_password("same", &a) && verify_password("same", &b));
    }

    #[test]
    fn malformed_hash_rejects() {
        assert!(!verify_password("x", "not-a-phc-string"));
    }
}
