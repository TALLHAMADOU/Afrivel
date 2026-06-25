//! Règles de nommage partagées (identifiants Rust, noms de tables).
//!
//! Hand-rollé pour préserver l'invariant « zéro dépendance runtime » : la même logique
//! est compilée dans la CLI et dans les macros procédurales.

/// Convertit un identifiant arbitraire en `snake_case`.
///
/// Gère `PascalCase`, `camelCase`, les frontières `kebab-case`/espaces, et les
/// séquences d'acronymes (`HTTPServer` → `http_server`).
pub fn snake_case(input: &str) -> String {
    let mut out = String::with_capacity(input.len() + 4);
    let chars: Vec<char> = input.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if c == '_' || c == '-' || c == ' ' {
            if !out.ends_with('_') && !out.is_empty() {
                out.push('_');
            }
            continue;
        }
        if c.is_ascii_uppercase() {
            let prev_lower =
                i > 0 && (chars[i - 1].is_ascii_lowercase() || chars[i - 1].is_ascii_digit());
            let next_lower = i + 1 < chars.len() && chars[i + 1].is_ascii_lowercase();
            let prev_upper = i > 0 && chars[i - 1].is_ascii_uppercase();
            if !out.is_empty() && !out.ends_with('_') && (prev_lower || (prev_upper && next_lower))
            {
                out.push('_');
            }
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out.trim_matches('_').to_string()
}

/// Convertit un identifiant arbitraire en `PascalCase`.
pub fn pascal_case(input: &str) -> String {
    snake_case(input)
        .split('_')
        .filter(|s| !s.is_empty())
        .map(capitalize)
        .collect()
}

/// Convertit un identifiant arbitraire en `camelCase`.
pub fn camel_case(input: &str) -> String {
    let pascal = pascal_case(input);
    let mut chars = pascal.chars();
    match chars.next() {
        Some(first) => first.to_ascii_lowercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

/// Met une majuscule à la première lettre d'un mot ASCII déjà en minuscules.
fn capitalize(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
        None => String::new(),
    }
}

/// Pluralise un mot anglais en `snake_case` selon des règles courantes.
///
/// Volontairement minimaliste (pas de dictionnaire d'irréguliers exhaustif) ; couvre
/// les cas réguliers et quelques irréguliers fréquents pour des noms de tables.
pub fn pluralize(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }
    // Irréguliers fréquents.
    match word {
        "person" => return "people".to_string(),
        "man" => return "men".to_string(),
        "woman" => return "women".to_string(),
        "child" => return "children".to_string(),
        "tooth" => return "teeth".to_string(),
        "foot" => return "feet".to_string(),
        "mouse" => return "mice".to_string(),
        _ => {}
    }
    let lower = word.to_ascii_lowercase();
    let ends_with = |suf: &str| lower.ends_with(suf);
    let is_vowel = |c: char| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u');

    if ends_with("s") || ends_with("x") || ends_with("z") || ends_with("ch") || ends_with("sh") {
        format!("{word}es")
    } else if ends_with("y") {
        let before_y = lower.chars().rev().nth(1);
        if before_y.map(is_vowel).unwrap_or(false) {
            format!("{word}s")
        } else {
            format!("{}ies", &word[..word.len() - 1])
        }
    } else if ends_with("f") {
        format!("{}ves", &word[..word.len() - 1])
    } else if ends_with("fe") {
        format!("{}ves", &word[..word.len() - 2])
    } else {
        format!("{word}s")
    }
}

/// Déduit le nom de table d'un modèle : `snake_case` puis pluriel (`UserProfile` →
/// `user_profiles`).
pub fn table_name(model: &str) -> String {
    let snake = snake_case(model);
    match snake.rsplit_once('_') {
        Some((head, last)) => format!("{head}_{}", pluralize(last)),
        None => pluralize(&snake),
    }
}

/// Vérifie qu'une chaîne est un identifiant Rust ASCII valide (lettre/`_` puis
/// alphanumériques/`_`).
pub fn is_valid_ident(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_case_variants() {
        assert_eq!(snake_case("UserProfile"), "user_profile");
        assert_eq!(snake_case("userProfile"), "user_profile");
        assert_eq!(snake_case("user-profile"), "user_profile");
        assert_eq!(snake_case("user profile"), "user_profile");
        assert_eq!(snake_case("HTTPServer"), "http_server");
        assert_eq!(snake_case("already_snake"), "already_snake");
        assert_eq!(snake_case("User"), "user");
    }

    #[test]
    fn pascal_and_camel() {
        assert_eq!(pascal_case("user_profile"), "UserProfile");
        assert_eq!(pascal_case("user-profile"), "UserProfile");
        assert_eq!(camel_case("user_profile"), "userProfile");
        assert_eq!(camel_case("User"), "user");
    }

    #[test]
    fn pluralization() {
        assert_eq!(pluralize("user"), "users");
        assert_eq!(pluralize("category"), "categories");
        assert_eq!(pluralize("day"), "days");
        assert_eq!(pluralize("box"), "boxes");
        assert_eq!(pluralize("class"), "classes");
        assert_eq!(pluralize("dish"), "dishes");
        assert_eq!(pluralize("leaf"), "leaves");
        assert_eq!(pluralize("knife"), "knives");
        assert_eq!(pluralize("person"), "people");
    }

    #[test]
    fn table_names() {
        assert_eq!(table_name("User"), "users");
        assert_eq!(table_name("UserProfile"), "user_profiles");
        assert_eq!(table_name("Category"), "categories");
    }

    #[test]
    fn idents() {
        assert!(is_valid_ident("user_id"));
        assert!(is_valid_ident("_x"));
        assert!(!is_valid_ident("2cool"));
        assert!(!is_valid_ident("a-b"));
        assert!(!is_valid_ident(""));
    }
}
