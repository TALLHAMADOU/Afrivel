//! Macros procédurales d'Afrivel.
//!
//! `#[derive(Model)]` enrichit un `Model` SeaORM (déjà dérivé via `DeriveEntityModel`)
//! d'une implémentation du trait ergonomique [`afrivel::orm::Model`] : il en extrait le
//! nom de table depuis l'attribut `#[sea_orm(table_name = "…")]` et branche le CRUD par
//! défaut. La logique de nommage/typage du DSL vit dans `afrivel-codegen` (M2/M3).

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Expr, LitStr, parse_macro_input};

/// Dérive l'implémentation du trait `afrivel::orm::Model` pour un `Model` SeaORM.
///
/// Requiert un attribut `#[sea_orm(table_name = "…")]` sur la struct (posé par
/// `DeriveEntityModel`).
#[proc_macro_derive(Model, attributes(sea_orm))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident.clone();

    let table = match extract_table_name(&input) {
        Ok(Some(table)) => table,
        Ok(None) => {
            return syn::Error::new_spanned(
                &ident,
                "#[derive(Model)] requiert un attribut `#[sea_orm(table_name = \"…\")]`",
            )
            .to_compile_error()
            .into();
        }
        Err(err) => return err.to_compile_error().into(),
    };

    quote! {
        impl ::afrivel::orm::Model for #ident {
            const TABLE: &'static str = #table;
        }
    }
    .into()
}

/// Extrait `table_name` depuis les attributs `#[sea_orm(...)]`, en ignorant proprement les
/// autres clés (`schema_name`, `rename_all`, …).
fn extract_table_name(input: &DeriveInput) -> syn::Result<Option<String>> {
    let mut table = None;
    for attr in &input.attrs {
        if !attr.path().is_ident("sea_orm") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("table_name") {
                let value: LitStr = meta.value()?.parse()?;
                table = Some(value.value());
                return Ok(());
            }
            // Consomme la valeur des autres clés pour ne pas échouer.
            if meta.input.peek(syn::Token![=]) {
                let _: Expr = meta.value()?.parse()?;
            }
            Ok(())
        })?;
    }
    Ok(table)
}
