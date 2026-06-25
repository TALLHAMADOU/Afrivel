//! Types de champs du DSL et leur correspondance Rust / SeaORM.

use crate::error::ParseError;

/// Type logique d'un champ, indépendant du SGBD.
///
/// Le mapping vers Rust et vers `sea_orm::ColumnType` est centralisé ici afin que la CLI
/// (génération de fichiers) et les macros (`#[derive(Model)]`) produisent un code cohérent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldType {
    /// Chaîne courte (`VARCHAR`).
    String,
    /// Texte long (`TEXT`).
    Text,
    /// Entier signé 32 bits.
    Integer,
    /// Entier signé 64 bits.
    BigInteger,
    /// Booléen.
    Boolean,
    /// Flottant double précision.
    Float,
    /// Décimal exact (montants monétaires).
    Decimal,
    /// Identifiant UUID.
    Uuid,
    /// Horodatage avec fuseau (UTC).
    Timestamp,
    /// Date sans heure.
    Date,
    /// Document JSON.
    Json,
}

impl FieldType {
    /// Analyse un type depuis le DSL (insensible à la casse, alias courants acceptés).
    pub fn parse(s: &str) -> Result<Self, ParseError> {
        let ty = match s.to_ascii_lowercase().as_str() {
            "string" | "varchar" | "str" => FieldType::String,
            "text" | "longtext" => FieldType::Text,
            "integer" | "int" | "i32" => FieldType::Integer,
            "biginteger" | "bigint" | "i64" | "bigserial" => FieldType::BigInteger,
            "boolean" | "bool" => FieldType::Boolean,
            "float" | "double" | "f64" | "real" => FieldType::Float,
            "decimal" | "money" | "numeric" => FieldType::Decimal,
            "uuid" => FieldType::Uuid,
            "timestamp" | "datetime" => FieldType::Timestamp,
            "date" => FieldType::Date,
            "json" | "jsonb" => FieldType::Json,
            _ => return Err(ParseError::UnknownType(s.to_string())),
        };
        Ok(ty)
    }

    /// Type Rust correspondant (côté entité, hors `Option`).
    ///
    /// Les types non primitifs réfèrent des crates que l'entité générée importe
    /// (`uuid`, `chrono`, `rust_decimal`, `serde_json`).
    pub fn rust_type(self) -> &'static str {
        match self {
            FieldType::String | FieldType::Text => "String",
            FieldType::Integer => "i32",
            FieldType::BigInteger => "i64",
            FieldType::Boolean => "bool",
            FieldType::Float => "f64",
            FieldType::Decimal => "Decimal",
            FieldType::Uuid => "Uuid",
            FieldType::Timestamp => "DateTimeUtc",
            FieldType::Date => "Date",
            FieldType::Json => "Json",
        }
    }

    /// Variante `sea_orm::ColumnType` correspondante (expression Rust à émettre).
    pub fn sea_column_type(self) -> &'static str {
        match self {
            FieldType::String => "ColumnType::String(StringLen::None)",
            FieldType::Text => "ColumnType::Text",
            FieldType::Integer => "ColumnType::Integer",
            FieldType::BigInteger => "ColumnType::BigInteger",
            FieldType::Boolean => "ColumnType::Boolean",
            FieldType::Float => "ColumnType::Double",
            FieldType::Decimal => "ColumnType::Decimal(None)",
            FieldType::Uuid => "ColumnType::Uuid",
            FieldType::Timestamp => "ColumnType::TimestampWithTimeZone",
            FieldType::Date => "ColumnType::Date",
            FieldType::Json => "ColumnType::JsonBinary",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_aliases() {
        assert_eq!(FieldType::parse("string").unwrap(), FieldType::String);
        assert_eq!(FieldType::parse("INT").unwrap(), FieldType::Integer);
        assert_eq!(FieldType::parse("bigint").unwrap(), FieldType::BigInteger);
        assert_eq!(FieldType::parse("bool").unwrap(), FieldType::Boolean);
        assert_eq!(FieldType::parse("jsonb").unwrap(), FieldType::Json);
    }

    #[test]
    fn rejects_unknown() {
        assert_eq!(
            FieldType::parse("blob"),
            Err(ParseError::UnknownType("blob".to_string()))
        );
    }

    #[test]
    fn maps_to_rust() {
        assert_eq!(FieldType::String.rust_type(), "String");
        assert_eq!(FieldType::BigInteger.rust_type(), "i64");
        assert_eq!(FieldType::Timestamp.rust_type(), "DateTimeUtc");
    }
}
