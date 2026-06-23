# Politique de sécurité

## Versions supportées

Afrivel est en **v0.0.1 (phase de design)**. Aucune version stable n'est encore publiée.

| Version | Supportée |
|---------|-----------|
| `0.0.x` | ⚠️ pré-version, aucune garantie de sécurité |
| `< 0.0.1` | ❌ |

Une fois la première version utilisable publiée, ce tableau précisera les branches recevant
des correctifs de sécurité.

## Signaler une vulnérabilité

**Ne crée pas d'issue publique** pour une faille de sécurité.

Signale-la de façon responsable via un **avis de sécurité privé GitHub** :

1. Va dans l'onglet **Security** du dépôt.
2. Clique sur **« Report a vulnerability »** (Privately report a vulnerability).
3. Décris la faille, son impact, et si possible une preuve de concept et des étapes de
   reproduction.

Tu peux t'attendre à :

- un **accusé de réception** sous quelques jours ;
- une **évaluation** de l'impact et de la sévérité ;
- une **coordination** sur le correctif et le calendrier de divulgation ;
- un **crédit** (si tu le souhaites) dans l'avis publié.

Merci de laisser un délai raisonnable de correction avant toute divulgation publique
(divulgation coordonnée).

## Portée

Cette politique couvre le code du framework Afrivel (crates `afrivel-*`) et la CLI. Le code
**généré** dans une application utilisateur relève de la responsabilité de cette application,
mais tout défaut provenant des **templates de génération** d'Afrivel entre dans cette portée.
