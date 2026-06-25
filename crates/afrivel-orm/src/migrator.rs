//! Agrégation et ordonnancement des migrations (DR-021 : tri par timestamp).
//!
//! Chaque module expose ses migrations ; le binaire `app` les agrège dans un unique
//! `Migrator`. L'ordre d'exécution est déterminé par le **timestamp** porté par le nom de
//! chaque migration (convention SeaORM `mYYYYMMDD_HHMMSS_description`), qui est
//! lexicographiquement croissant — donc chronologique.

use sea_orm_migration::MigrationTrait;

/// Trie un ensemble de migrations par leur nom (= timestamp chronologique).
///
/// Source unique de l'ordre : appelée par le `Migrator` agrégé du projet généré pour
/// fusionner les migrations de tous les modules de façon déterministe.
pub fn sorted(mut migrations: Vec<Box<dyn MigrationTrait>>) -> Vec<Box<dyn MigrationTrait>> {
    migrations.sort_by(|a, b| a.name().cmp(b.name()));
    migrations
}

/// Variante pure (sur les noms) — c'est l'invariant testé : un tri lexical des noms de
/// migration produit l'ordre chronologique attendu.
pub fn sort_names(names: &mut [String]) {
    names.sort();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexical_sort_is_chronological() {
        let mut names = vec![
            "m20240115_090000_create_roles".to_string(),
            "m20240101_000001_create_users".to_string(),
            "m20240101_000002_create_permissions".to_string(),
            "m20231231_235959_init".to_string(),
        ];
        sort_names(&mut names);
        assert_eq!(
            names,
            vec![
                "m20231231_235959_init".to_string(),
                "m20240101_000001_create_users".to_string(),
                "m20240101_000002_create_permissions".to_string(),
                "m20240115_090000_create_roles".to_string(),
            ]
        );
    }
}
