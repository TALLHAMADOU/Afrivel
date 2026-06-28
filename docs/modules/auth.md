# Module — Auth

Module de référence d'Afrivel : **inscription, connexion (JWT) et RBAC**, écrit à la main en
Clean Architecture dans [`examples/demo`](../../examples/demo). Il s'appuie sur les primitives
d'authentification du noyau, exposées sous `afrivel::auth`.

## Primitives du noyau (`afrivel::auth`)

| Élément | Rôle |
|---------|------|
| `hashing::hash_password` / `verify_password` | Hachage Argon2id (chaînes PHC). |
| `jwt::{JwtSecret, Claims, encode, decode}` | Émission/validation JWT HS256 (`jsonwebtoken`). |
| `AuthUser` | Extracteur Axum : valide le Bearer et expose `id`, `roles`, `permissions`. |
| `Guard` + `Authorized<G>` | RBAC typé : un extracteur qui échoue (403) si la politique `G` refuse l'utilisateur. |
| `authenticate` | Middleware optionnel injectant `AuthUser` dans les extensions. |

`AuthUser`/`Authorized<G>` lisent le `JwtSecret` depuis les extensions de la requête : il faut
le fournir à l'application via `Application::provide(secret)`.

## Couches du module (Clean Architecture)

```
contracts/   port UserRepository (UserRef, Credentials)
domain/      entités User/Role/Permission + pivots N-N (SeaORM) ; AuthService (register/login)
http/        requests validés · resources · controllers · routes
infra/       repositories : InMemory (tests) + SeaORM (production)
guards.rs    AdminGuard (exige le rôle « admin »)
migration.rs création des tables auth
```

Les autres modules ne dépendent que de `contracts` ; l'implémentation est injectée via
`Arc<dyn UserRepository>` (inversion de dépendance).

## Routes

| Méthode | Chemin | Accès | Réponse |
|---------|--------|-------|---------|
| `POST` | `/auth/register` | public | `201` + `{ id, email }` |
| `POST` | `/auth/login` | public | `200` + `{ token, token_type }` |
| `GET` | `/auth/me` | Bearer | `200` + `{ id, roles, permissions }` |
| `GET` | `/auth/admin` | Bearer + rôle `admin` | `200` (sinon `403`) |

Erreurs : `422` (validation : email invalide, mot de passe < 8), `401` (identifiants
inconnus/faux — sans oracle d'énumération de comptes), `403` (rôle manquant).

## Câblage

```rust
let users: Arc<dyn UserRepository> = Arc::new(SeaOrmUserRepository::new(db.clone()));
let service = AuthService::new(users, secret.clone());
Application::new()
    .register(AuthModule)
    .provide(db)
    .provide(secret)   // requis par AuthUser / Authorized<G>
    .provide(service);
```

## Définir un guard RBAC

```rust
pub struct AdminGuard;
impl Guard for AdminGuard {
    fn authorize(user: &AuthUser) -> Result<()> {
        user.require_role("admin")
    }
}

async fn admin(guard: Authorized<AdminGuard>) -> Json<ProfileResource> {
    let user = guard.user; // déjà autorisé
    /* ... */
}
```

## Tests

- **Unitaires** (`AuthService`) : register dupliqué → `Validation` ; login mauvais mot de
  passe / email inconnu → `Unauthorized`.
- **Intégration HTTP** (en mémoire) : flux register→login→protégé, RBAC `403`→`200`, validation.
- **Smoke Postgres** (CI) : migrations réelles + flux HTTP, activé par `DATABASE_URL`.
