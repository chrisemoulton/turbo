use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, FieldsNamed, FieldsUnnamed};
use turbo_tasks_macros_shared::{generate_destructuring, match_expansion};

fn ignore_field(_field: &Field) -> bool {
    false
}

/// This macro generates the implementation of the `DeterministicHash` trait for
/// a given type.
///
/// This requires that every contained value also implement `DeterministicHash`.
pub fn derive_deterministic_hash(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let ident = &derive_input.ident;
    let hash_logic = match_expansion(&derive_input, &hash_named, &hash_unnamed, &hash_unit);

    quote! {
        impl turbo_tasks_hash::DeterministicHash for #ident {
            fn deterministic_hash<H: std::hash::Hasher>(&self, __state__: &mut H) {
                #hash_logic
            }
        }
    }
    .into()
}

/// Hashes a struct or enum variant with named fields (e.g. `struct Foo {
/// bar: u32 }`, `Foo::Bar { baz: u32 }`).
fn hash_named(_ident: &Ident, fields: &FieldsNamed) -> (TokenStream2, TokenStream2) {
    let (captures, fields_idents) = generate_destructuring(fields.named.iter(), &ignore_field);
    (
        captures,
        quote! {
            {#(
                #fields_idents.deterministic_hash(__state__);
            )*}
        },
    )
}

/// Hashes a struct or enum variant with unnamed fields (e.g. `struct
/// Foo(u32)`, `Foo::Bar(u32)`).
fn hash_unnamed(_ident: &Ident, fields: &FieldsUnnamed) -> (TokenStream2, TokenStream2) {
    let (captures, fields_idents) = generate_destructuring(fields.unnamed.iter(), &ignore_field);
    (
        captures,
        quote! {
            {#(
                #fields_idents.deterministic_hash(__state__);
            )*}
        },
    )
}

/// Hashes a unit struct or enum variant (e.g. `struct Foo;`, `Foo::Bar`).
fn hash_unit(_ident: &Ident) -> (TokenStream2, TokenStream2) {
    (quote! {}, quote! { { } })
}
