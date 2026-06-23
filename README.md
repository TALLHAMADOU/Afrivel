<div align="center">

# Afrivel

**L'expérience Laravel, propulsée par Rust.**

[Documentation](./docs/README.md) · [Roadmap](./docs/ROADMAP.md) · [Decisions](./docs/DECISIONS.md) · [Contribuer](#contribuer)

</div>

---

> ⚠️ **Statut : v0.0.1 en cours de design.** Le framework n'est pas encore utilisable. Cette page décrit l'expérience cible.

## À propos d'Afrivel

Afrivel est un framework web à la syntaxe expressive et élégante, propulsé par Rust. Nous
croyons que le développement doit être une expérience agréable et créative pour être réellement
épanouissante. Afrivel allège la douleur du développement en simplifiant les tâches communes à
la plupart des projets web, tout en bénéficiant de la performance, de la sécurité mémoire et de
la concurrence de Rust :

- Moteur de routing simple et rapide (au-dessus d'Axum).
- Génération de modules métier complets en une commande (`afrivel make:module`).
- Conteneur d'injection de dépendances puissant, vérifié à la compilation.
- ORM de base de données expressif et intuitif (au-dessus de SeaORM).
- Migrations de schéma agnostiques de la base, ordonnées de façon déterministe.
- Gestion d'erreurs unifiée mappée vers des réponses HTTP propres.
- Traitement de jobs en arrière-plan robuste *(prévu)*.
- Diffusion d'événements temps réel *(prévu)*.

Afrivel est accessible, puissant, et fournit les outils nécessaires aux applications larges et
robustes — grâce à une architecture **module-centric** en **Clean Architecture** par défaut.

```bash
afrivel make:module Auth --model User:name:string,email:string:unique,password:string
```

Une commande génère une crate de module encapsulée et complète : modèles, requests, controllers,
services, interfaces, repositories, resources, migrations, routes et tests — **toujours compilables**.

## Apprendre Afrivel

Afrivel vise une documentation claire et exhaustive pour démarrer en quelques minutes. La
documentation de conception et d'architecture vit dans [`/docs`](./docs/README.md) :

- [DESIGN.md](./docs/DESIGN.md) — vision, hypothèses, périmètre.
- [ARCHITECTURE.md](./docs/ARCHITECTURE.md) — workspace, modules, Clean Architecture.
- [CLI.md](./docs/CLI.md) — référence complète de la CLI.
- [ROADMAP.md](./docs/ROADMAP.md) — feuille de route.

## Développement assisté par agents (Agentic Development)

La structure prévisible et les conventions d'Afrivel le rendent idéal pour les agents de
codage IA comme **Claude Code**, **Cursor** et **GitHub Copilot**. L'enregistrement explicite
(pas de réflexion runtime), la génération de code déterministe et la garantie « sortie toujours
compilable » donnent aux agents un terrain fiable : chaque module suit la même structure en
couches, et chaque commande `make:*` produit du code qui compile.

## Contribuer

Merci d'envisager de contribuer à Afrivel ! Afrivel est open source et accueille toutes les
contributions : code, documentation, tests, traductions, idées. Commence par la documentation
de conception dans [`/docs`](./docs/README.md), puis consulte les
[décisions d'architecture](./docs/DECISIONS.md).

## Code de conduite

Afin que la communauté Afrivel reste accueillante pour tous, merci de lire et de respecter le
Code de conduite *(à publier : `CODE_OF_CONDUCT.md`, basé sur le Contributor Covenant)*.

## Vulnérabilités de sécurité

Si tu découvres une vulnérabilité de sécurité dans Afrivel, merci de la signaler de façon
responsable via un **avis de sécurité privé GitHub**
(`Security → Report a vulnerability` sur le dépôt) plutôt que par une issue publique. Toutes les
vulnérabilités seront traitées rapidement.

## Licence

Afrivel est un logiciel open source sous **double licence MIT OR Apache-2.0**, au choix — le
standard de l'écosystème Rust. Voir [`LICENSE-MIT`](./LICENSE-MIT) et [`LICENSE-APACHE`](./LICENSE-APACHE).

> Toolchain : Rust **stable**, edition **2024** (MSRV 1.85+).
