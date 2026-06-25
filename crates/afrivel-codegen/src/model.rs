//! Parsing du DSL `--model` en une `ModelSpec` structurée.
//!
//! Grammaire (informelle) :
//!
//! ```text
//! spec   := Model ':' field (',' field)*
//! field  := name ':' type (':' modifier)*
//! modifier := 'unique' | 'nullable' | 'index'
//!           | 'default=' value
//!           | 'fk=' Table
//! ```
//!
//! Exemple : `User:name:string,email:string:unique,password:string,role_id:bigint:fk=Role`.

use crate::error::ParseError;
use crate::naming;
use crate::types::FieldType;

/// Modificateurs applicables à un champ.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Modifiers {
    /// Contrainte d'unicité.
    pub unique: bool,
    /// Colonne nullable (`Option<T>` côté Rust).
    pub nullable: bool,
    /// Index simple sur la colonne.
    pub index: bool,
    /// Valeur par défaut (expression SQL brute, telle quelle).
    pub default: Option<String>,
    /// Clé étrangère : nom du modèle référencé (la table cible en est dérivée).
    pub foreign_key: Option<String>,
}

/// Un champ de modèle : nom `snake_case`, type et modificateurs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    /// Nom de colonne / champ (`snake_case`).
    pub name: String,
    /// Type logique.
    pub ty: FieldType,
    /// Modificateurs.
    pub modifiers: Modifiers,
}

impl Field {
    /// Modèle référencé par la clé étrangère, le cas échéant.
    pub fn references(&self) -> Option<&str> {
        self.modifiers.foreign_key.as_deref()
    }
}

/// Spécification complète d'un modèle issue du DSL.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSpec {
    /// Nom du modèle en `PascalCase` (ex. `User`).
    pub name: String,
    /// Champs déclarés (hors `id`/timestamps implicites).
    pub fields: Vec<Field>,
}

impl ModelSpec {
    /// Analyse une spécification complète (`Model:field:type,...`).
    pub fn parse(spec: &str) -> Result<Self, ParseError> {
        let spec = spec.trim();
        if spec.is_empty() {
            return Err(ParseError::Empty);
        }
        let (raw_name, rest) = match spec.split_once(':') {
            Some((name, rest)) => (name.trim(), rest),
            None => (spec, ""),
        };
        if !naming::is_valid_ident(raw_name) {
            return Err(ParseError::InvalidModelName(raw_name.to_string()));
        }
        let name = naming::pascal_case(raw_name);

        let mut fields = Vec::new();
        for chunk in rest.split(',') {
            let chunk = chunk.trim();
            if chunk.is_empty() {
                continue;
            }
            let field = parse_field(chunk)?;
            if fields.iter().any(|f: &Field| f.name == field.name) {
                return Err(ParseError::DuplicateField(field.name));
            }
            fields.push(field);
        }
        Ok(ModelSpec { name, fields })
    }

    /// Nom de table dérivé (`snake_case` pluriel).
    pub fn table_name(&self) -> String {
        naming::table_name(&self.name)
    }

    /// Nom de module/fichier (`snake_case` singulier).
    pub fn snake_name(&self) -> String {
        naming::snake_case(&self.name)
    }
}

fn parse_field(chunk: &str) -> Result<Field, ParseError> {
    let mut parts = chunk.split(':');
    let raw_name = parts
        .next()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ParseError::MalformedField(chunk.to_string()))?;
    if !naming::is_valid_ident(raw_name) {
        return Err(ParseError::InvalidFieldName(raw_name.to_string()));
    }
    let name = naming::snake_case(raw_name);

    let raw_type = parts
        .next()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ParseError::MalformedField(chunk.to_string()))?;
    let ty = FieldType::parse(raw_type)?;

    let mut modifiers = Modifiers::default();
    for raw in parts {
        let m = raw.trim();
        if m.is_empty() {
            continue;
        }
        if let Some(value) = m.strip_prefix("default=") {
            modifiers.default = Some(value.trim().to_string());
        } else if let Some(target) = m.strip_prefix("fk=") {
            let target = target.trim();
            if !naming::is_valid_ident(target) {
                return Err(ParseError::InvalidModifier(m.to_string()));
            }
            modifiers.foreign_key = Some(naming::pascal_case(target));
        } else {
            match m.to_ascii_lowercase().as_str() {
                "unique" => modifiers.unique = true,
                "nullable" => modifiers.nullable = true,
                "index" => modifiers.index = true,
                _ => return Err(ParseError::InvalidModifier(m.to_string())),
            }
        }
    }
    Ok(Field {
        name,
        ty,
        modifiers,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_spec() {
        let spec =
            ModelSpec::parse("User:name:string,email:string:unique,password:string").unwrap();
        assert_eq!(spec.name, "User");
        assert_eq!(spec.table_name(), "users");
        assert_eq!(spec.fields.len(), 3);

        let email = &spec.fields[1];
        assert_eq!(email.name, "email");
        assert_eq!(email.ty, FieldType::String);
        assert!(email.modifiers.unique);
        assert!(!email.modifiers.nullable);
    }

    #[test]
    fn parses_modifiers_and_fk() {
        let spec = ModelSpec::parse(
            "Post:title:string,body:text:nullable,views:int:default=0,author_id:bigint:fk=User",
        )
        .unwrap();
        assert!(spec.fields[1].modifiers.nullable);
        assert_eq!(spec.fields[2].modifiers.default.as_deref(), Some("0"));
        assert_eq!(spec.fields[3].references(), Some("User"));
    }

    #[test]
    fn normalises_names() {
        let spec = ModelSpec::parse("blogPost:postTitle:string").unwrap();
        assert_eq!(spec.name, "BlogPost");
        assert_eq!(spec.table_name(), "blog_posts");
        assert_eq!(spec.fields[0].name, "post_title");
    }

    #[test]
    fn model_without_fields() {
        let spec = ModelSpec::parse("Tag").unwrap();
        assert_eq!(spec.name, "Tag");
        assert!(spec.fields.is_empty());
    }

    #[test]
    fn rejects_empty() {
        assert_eq!(ModelSpec::parse("   "), Err(ParseError::Empty));
    }

    #[test]
    fn rejects_bad_model_name() {
        assert!(matches!(
            ModelSpec::parse("2User:name:string"),
            Err(ParseError::InvalidModelName(_))
        ));
    }

    #[test]
    fn rejects_unknown_type() {
        assert!(matches!(
            ModelSpec::parse("User:name:blob"),
            Err(ParseError::UnknownType(_))
        ));
    }

    #[test]
    fn rejects_unknown_modifier() {
        assert!(matches!(
            ModelSpec::parse("User:name:string:weird"),
            Err(ParseError::InvalidModifier(_))
        ));
    }

    #[test]
    fn rejects_duplicate_field() {
        assert!(matches!(
            ModelSpec::parse("User:name:string,name:text"),
            Err(ParseError::DuplicateField(_))
        ));
    }

    #[test]
    fn rejects_missing_type() {
        assert!(matches!(
            ModelSpec::parse("User:name"),
            Err(ParseError::MalformedField(_))
        ));
    }
}
