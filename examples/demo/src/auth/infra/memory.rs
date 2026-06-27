//! Dépôt utilisateur **en mémoire** : implémentation du port [`UserRepository`] pour les
//! tests (flux register→login→protégé) sans base de données.

use std::collections::HashMap;
use std::sync::Mutex;

use afrivel::{Error, Result};
use async_trait::async_trait;

use crate::auth::contracts::{Credentials, UserRef, UserRepository};

/// Dépôt en mémoire, thread-safe.
#[derive(Default)]
pub struct InMemoryUserRepository {
    inner: Mutex<State>,
}

#[derive(Default)]
struct State {
    next_id: u64,
    by_email: HashMap<String, Credentials>,
}

impl InMemoryUserRepository {
    /// Crée un dépôt vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Helper de test : accorde des rôles/permissions à un utilisateur existant.
    pub fn grant(&self, email: &str, roles: &[&str], permissions: &[&str]) {
        let mut state = self.inner.lock().expect("lock");
        if let Some(creds) = state.by_email.get_mut(email) {
            creds.roles = roles.iter().map(|r| r.to_string()).collect();
            creds.permissions = permissions.iter().map(|p| p.to_string()).collect();
        }
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn create(&self, email: &str, password_hash: &str) -> Result<UserRef> {
        let mut state = self.inner.lock().expect("lock");
        if state.by_email.contains_key(email) {
            return Err(Error::Validation("email déjà utilisé".into()));
        }
        state.next_id += 1;
        let id = state.next_id.to_string();
        let creds = Credentials {
            id: id.clone(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            roles: Vec::new(),
            permissions: Vec::new(),
        };
        state.by_email.insert(email.to_string(), creds);
        Ok(UserRef {
            id,
            email: email.to_string(),
        })
    }

    async fn by_email(&self, email: &str) -> Result<Option<Credentials>> {
        Ok(self
            .inner
            .lock()
            .expect("lock")
            .by_email
            .get(email)
            .cloned())
    }
}
