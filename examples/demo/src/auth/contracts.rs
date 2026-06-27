//! Contrats du module Auth : surface publique (traits) consommée par les services et par
//! d'autres modules. Aucune dépendance vers l'infrastructure (Clean Architecture).

use afrivel::Result;
use async_trait::async_trait;

/// Référence publique d'un utilisateur — **seul** type qu'un autre module devrait connaître.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserRef {
    /// Identifiant stable de l'utilisateur.
    pub id: String,
    /// Adresse e-mail (identifiant de connexion).
    pub email: String,
}

/// Données d'authentification d'un utilisateur (usage interne au module Auth).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credentials {
    /// Identifiant de l'utilisateur.
    pub id: String,
    /// Adresse e-mail.
    pub email: String,
    /// Hachage Argon2 du mot de passe (chaîne PHC).
    pub password_hash: String,
    /// Rôles accordés.
    pub roles: Vec<String>,
    /// Permissions accordées (agrégées depuis les rôles).
    pub permissions: Vec<String>,
}

/// Port de persistance des utilisateurs : implémenté par l'infra (SeaORM, ou en mémoire
/// pour les tests). Les services dépendent de **ce trait**, jamais d'une implémentation.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Persiste un nouvel utilisateur et renvoie sa référence publique.
    async fn create(&self, email: &str, password_hash: &str) -> Result<UserRef>;

    /// Recherche un utilisateur par e-mail, avec ses identifiants d'authentification.
    async fn by_email(&self, email: &str) -> Result<Option<Credentials>>;
}
