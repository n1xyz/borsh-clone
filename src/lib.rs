#![deny(missing_docs)]

//! `borsh-clone` provides a derive macro `BorshClone` that implements `Clone` for a type
//! by serializing and deserializing it using `borsh`.
//!
//! # Example
//!
//! ```rust
//! use borsh::{BorshDeserialize, BorshSerialize};
//! use borsh_clone::BorshClone;
//!
//! #[derive(Debug, PartialEq, Eq, BorshSerialize, BorshDeserialize, BorshClone)]
//! struct MyStruct {
//!     id: u64,
//!     name: String,
//! }
//!
//! let original = MyStruct {
//!     id: 42,
//!     name: "Borsh".to_string(),
//! };
//! let cloned = original.clone();
//! assert_eq!(original, cloned);
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::Parser, parse_macro_input, parse_quote, punctuated::Punctuated, Attribute, DeriveInput,
    GenericParam, Generics, LitStr, Path, Token, WherePredicate,
};

#[derive(Default)]
struct ParsedAttrs {
    borsh_path: Option<Path>,
    skip_bounds: bool,
    custom_bounds: Vec<WherePredicate>,
}

/// Derives `Clone` for a type using `borsh::to_vec` and `borsh::from_slice`.
///
/// # Attributes
///
/// - `#[borsh(crate = "...")]` or `#[borsh_clone(crate = "...")]`: Custom path to `borsh` crate.
/// - `#[borsh_clone(skip_bounds)]`: Disables automatic addition of `BorshSerialize + BorshDeserialize` bounds to generic type parameters.
/// - `#[borsh_clone(bound = "T: MyBound")]`: Adds custom bounds to the `where` clause.
#[proc_macro_derive(BorshClone, attributes(borsh, borsh_clone, borsh_crate))]
pub fn borsh_clone_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let parsed_attrs = parse_attributes(&input.attrs);
    let borsh_path = parsed_attrs
        .borsh_path
        .clone()
        .unwrap_or_else(|| parse_quote!(::borsh));

    let generics = apply_generics(input.generics, &borsh_path, &parsed_attrs);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics ::core::clone::Clone for #name #ty_generics #where_clause {
            fn clone(&self) -> Self {
                let bytes = #borsh_path::to_vec(self)
                    .expect("Failed to serialize in BorshClone::clone");
                #borsh_path::from_slice(&bytes)
                    .expect("Failed to deserialize in BorshClone::clone")
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_attributes(attrs: &[Attribute]) -> ParsedAttrs {
    let mut result = ParsedAttrs::default();

    for attr in attrs {
        if attr.path().is_ident("borsh") || attr.path().is_ident("borsh_clone") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("crate") {
                    if let Ok(value) = meta.value() {
                        if let Ok(s) = value.parse::<LitStr>() {
                            if let Ok(path) = s.parse::<Path>() {
                                result.borsh_path = Some(path);
                                return Ok(());
                            }
                        }
                        if let Ok(path) = value.parse::<Path>() {
                            result.borsh_path = Some(path);
                            return Ok(());
                        }
                    }
                } else if meta.path.is_ident("skip_bounds") {
                    result.skip_bounds = true;
                } else if meta.path.is_ident("bound") {
                    if let Ok(value) = meta.value() {
                        if let Ok(s) = value.parse::<LitStr>() {
                            let parser = Punctuated::<WherePredicate, Token![,]>::parse_terminated;
                            if let Ok(preds) = parser.parse_str(&s.value()) {
                                result.custom_bounds.extend(preds);
                            }
                        }
                    }
                }
                Ok(())
            });
        } else if attr.path().is_ident("borsh_crate") {
            if let Ok(s) = attr.parse_args::<LitStr>() {
                if let Ok(path) = s.parse::<Path>() {
                    result.borsh_path = Some(path);
                }
            } else if let Ok(path) = attr.parse_args::<Path>() {
                result.borsh_path = Some(path);
            }
        }
    }

    result
}

fn apply_generics(
    mut generics: Generics,
    borsh_path: &Path,
    parsed_attrs: &ParsedAttrs,
) -> Generics {
    if !parsed_attrs.skip_bounds && parsed_attrs.custom_bounds.is_empty() {
        for param in &mut generics.params {
            if let GenericParam::Type(ref mut type_param) = param {
                type_param
                    .bounds
                    .push(parse_quote!(#borsh_path::BorshSerialize));
                type_param
                    .bounds
                    .push(parse_quote!(#borsh_path::BorshDeserialize));
            }
        }
    }

    if !parsed_attrs.custom_bounds.is_empty() {
        let where_clause = generics.make_where_clause();
        for bound in &parsed_attrs.custom_bounds {
            where_clause.predicates.push(bound.clone());
        }
    }

    generics
}
