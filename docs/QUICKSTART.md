# Quickstart — Afrivel

> ⚠️ **v0.0.1.** Le module Auth de référence vit dans [`examples/demo`](../examples/demo) ;
> il sert aussi de smoke test end-to-end en CI (migrations + flux HTTP sur Postgres).

## 1. Installer la CLI

```sh
cargo install afrivel
```

Le binaire global `afrivel` crée des projets et délègue le runtime au binaire de l'app
(`cargo run -p app`). Voir la [référence CLI complète](./CLI.md).

## 2. Créer un projet

```sh
afrivel new mon-app
cd mon-app
afrivel make:module blog --model "title:string body:text published:bool"
```

`new` génère un Cargo workspace (`app/`, `Afrivel.toml`, `migrator.rs`, `registry.rs`) ;
`make:module` ajoute un module en Clean Architecture (couches `http → services → contracts
← repositories`) et l'enregistre dans le double registre (`Afrivel.toml` + `registry.rs`).

## 3. Base de données & migrations

```sh
export DATABASE_URL="postgres://user:pass@localhost:5432/mon_app"
afrivel migrate          # applique les migrations en attente
afrivel migrate:status   # état des migrations
afrivel migrate:fresh    # recrée tout le schéma (dev/CI)
```

## 4. Lancer le serveur

```sh
afrivel dev              # auto-reload (watch + recompile + restart)
afrivel serve --port 3000
afrivel route:list       # routes enregistrées (sans démarrer le serveur)
```

## 5. Le flux Auth en pratique

Le module [`auth`](./modules/auth.md) expose, prêt à l'emploi :

```sh
# Inscription
curl -X POST localhost:3000/auth/register \
  -H 'content-type: application/json' \
  -d '{"email":"alice@example.com","password":"supersecret"}'

# Connexion → renvoie un jeton JWT
curl -X POST localhost:3000/auth/login \
  -H 'content-type: application/json' \
  -d '{"email":"alice@example.com","password":"supersecret"}'

# Accès protégé (Bearer)
curl localhost:3000/auth/me -H "authorization: Bearer <token>"
```

## Reproduire la démo localement

```sh
git clone <repo> && cd Afrivel
docker run -d --name pg -e POSTGRES_USER=afrivel -e POSTGRES_PASSWORD=afrivel \
  -e POSTGRES_DB=afrivel_test -p 5432:5432 postgres:16
export DATABASE_URL="postgres://afrivel:afrivel@localhost:5432/afrivel_test"
cargo run -p demo -- migrate:fresh
cargo run -p demo -- serve            # API sur http://127.0.0.1:3000
```

Sans `DATABASE_URL`, les tests d'intégration en mémoire couvrent le même flux :
`cargo test -p demo`.
