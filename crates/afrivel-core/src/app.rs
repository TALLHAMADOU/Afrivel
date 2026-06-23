//! Construction de l'application : composition des modules, injection de dépendances, service HTTP.

use std::net::SocketAddr;

use axum::{Extension, Router};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

use crate::error::Result;

/// Une unité fonctionnelle autonome exposant des routes.
///
/// Chaque module généré par `afrivel make:module` implémente ce contrat ; son enregistrement
/// dans l'[`Application`] est explicite (aucune réflexion runtime).
pub trait Module {
    /// Nom court du module (ex. `"auth"`).
    fn name(&self) -> &'static str;

    /// Routeur exposant les routes du module.
    fn routes(&self) -> Router;
}

/// Constructeur de l'application Afrivel.
///
/// On y enregistre des modules ([`Application::register`]) et des dépendances partagées
/// ([`Application::provide`]), puis on obtient un [`Router`] ([`Application::into_router`]) ou
/// on lance le serveur ([`Application::serve`]).
pub struct Application {
    router: Router,
    modules: Vec<&'static str>,
    layers: Vec<Box<dyn FnOnce(Router) -> Router>>,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            router: Router::new(),
            modules: Vec::new(),
            layers: Vec::new(),
        }
    }
}

impl Application {
    /// Crée une application vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enregistre un module et fusionne ses routes.
    pub fn register<M: Module>(mut self, module: M) -> Self {
        self.modules.push(module.name());
        self.router = self.router.merge(module.routes());
        self
    }

    /// Fournit une dépendance partagée, injectée dans tous les handlers via `Extension<T>`.
    ///
    /// C'est le mécanisme de DI d'Afrivel : on enregistre typiquement des `Arc<dyn Trait>`
    /// (un repository) consommés par les services. Appliqué à l'ensemble du routeur, quel que
    /// soit l'ordre d'appel par rapport à [`Application::register`].
    pub fn provide<T>(mut self, value: T) -> Self
    where
        T: Clone + Send + Sync + 'static,
    {
        self.layers
            .push(Box::new(move |router| router.layer(Extension(value))));
        self
    }

    /// Noms des modules enregistrés, dans l'ordre.
    pub fn modules(&self) -> &[&'static str] {
        &self.modules
    }

    /// Finalise et renvoie le [`Router`] (dépendances injectées + couche de trace HTTP).
    pub fn into_router(self) -> Router {
        let mut router = self.router;
        for layer in self.layers {
            router = layer(router);
        }
        router.layer(TraceLayer::new_for_http())
    }

    /// Lance le serveur HTTP sur l'adresse donnée.
    pub async fn serve(self, addr: impl Into<SocketAddr>) -> Result<()> {
        let addr = addr.into();
        let listener = TcpListener::bind(addr).await?;
        tracing::info!("Afrivel en écoute sur http://{addr}");
        axum::serve(listener, self.into_router()).await?;
        Ok(())
    }
}
