//! Erreurs de parsing du DSL `--model`.

use std::fmt;

/// Erreur produite lors de l'analyse d'une spécification de modèle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// La spécification est vide.
    Empty,
    /// Le nom du modèle est invalide (vide ou identifiant non valide).
    InvalidModelName(String),
    /// Un champ est mal formé (segment vide, etc.).
    MalformedField(String),
    /// Le nom d'un champ est invalide.
    InvalidFieldName(String),
    /// Le type d'un champ est inconnu.
    UnknownType(String),
    /// Un modificateur est inconnu ou mal formé.
    InvalidModifier(String),
    /// Deux champs portent le même nom.
    DuplicateField(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::Empty => write!(f, "spécification de modèle vide"),
            ParseError::InvalidModelName(s) => write!(f, "nom de modèle invalide : `{s}`"),
            ParseError::MalformedField(s) => write!(f, "champ mal formé : `{s}`"),
            ParseError::InvalidFieldName(s) => write!(f, "nom de champ invalide : `{s}`"),
            ParseError::UnknownType(s) => write!(f, "type inconnu : `{s}`"),
            ParseError::InvalidModifier(s) => write!(f, "modificateur invalide : `{s}`"),
            ParseError::DuplicateField(s) => write!(f, "champ en double : `{s}`"),
        }
    }
}

impl std::error::Error for ParseError {}
