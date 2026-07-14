//! Derive procedural usado para compor a saída de apresentação.
//!
//! O macro espera uma struct com campos nomeados. Cada campo recebe
//! `#[present(order = N)]`. Ele gera uma implementação de
//! `crate::presentation::Present<C>` que consome os campos em ordem crescente.

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote,
    spanned::Spanned,
    Data, DeriveInput, Error, Field, Fields, LitInt, Result,
};

/// Gera a composição de presenters para uma struct de saída.
///
/// Exemplo:
///
/// ```ignore
/// #[derive(PresentOutput)]
/// struct PresentationOutput {
///     #[present(order = 10)]
///     spawns: SpawnCommands,
///     #[present(order = 20)]
///     spatial: SpatialPatches,
///     #[present(order = 90)]
///     despawns: DespawnCommands,
/// }
/// ```
///
/// A expansão chama `Present::present` em ordem 10, 20 e 90.
#[proc_macro_derive(PresentOutput, attributes(present))]
pub fn derive_present_output(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand_present_output(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn expand_present_output(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let name = input.ident;
    let original_generics = input.generics;

    let fields = match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => fields.named,
            _ => {
                return Err(Error::new(
                    name.span(),
                    "PresentOutput exige uma struct com campos nomeados",
                ));
            }
        },
        _ => {
            return Err(Error::new(
                name.span(),
                "PresentOutput só pode ser derivado para structs",
            ));
        }
    };

    let mut ordered_fields = Vec::with_capacity(fields.len());
    let mut used_orders: HashMap<u32, syn::Ident> = HashMap::new();

    for field in fields {
        let ident = field.ident.clone().ok_or_else(|| {
            Error::new(field.span(), "campo nomeado esperado")
        })?;
        let order = parse_order(&field)?;

        if let Some(previous) = used_orders.insert(order, ident.clone()) {
            return Err(Error::new(
                ident.span(),
                format!(
                    "ordem {order} duplicada: os campos `{previous}` e `{ident}` não podem ocupar a mesma fase"
                ),
            ));
        }

        ordered_fields.push((order, ident, field.ty));
    }

    ordered_fields.sort_by_key(|(order, _, _)| *order);

    let field_idents: Vec<_> = ordered_fields
        .iter()
        .map(|(_, ident, _)| ident)
        .collect();
    let field_types: Vec<_> = ordered_fields
        .iter()
        .map(|(_, _, ty)| ty)
        .collect();

    // O impl é genérico sobre o contexto. Assim o derive não conhece Godot:
    // ele só exige que cada campo saiba se apresentar naquele contexto.
    let mut impl_generics = original_generics.clone();
    impl_generics
        .params
        .push(parse_quote!(__PresentationContext));

    {
        let where_clause = impl_generics.make_where_clause();
        for field_type in &field_types {
            where_clause.predicates.push(parse_quote!(
                #field_type: crate::presentation::Present<__PresentationContext>
            ));
        }
    }

    let (impl_generics_tokens, _, where_clause) =
        impl_generics.split_for_impl();
    let (_, type_generics, _) = original_generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics_tokens
            crate::presentation::Present<__PresentationContext>
            for #name #type_generics
            #where_clause
        {
            fn present(
                self,
                context: &mut __PresentationContext,
            ) {
                let Self { #(#field_idents),* } = self;

                #(
                    <#field_types as crate::presentation::Present<__PresentationContext>>::present(
                        #field_idents,
                        context,
                    );
                )*
            }
        }
    })
}

fn parse_order(field: &Field) -> Result<u32> {
    let mut order = None;
    let mut found_present_attribute = false;

    for attribute in field
        .attrs
        .iter()
        .filter(|attribute| attribute.path().is_ident("present"))
    {
        found_present_attribute = true;

        attribute.parse_nested_meta(|meta| {
            if !meta.path.is_ident("order") {
                return Err(meta.error(
                    "atributo desconhecido; use somente `order = N`",
                ));
            }

            if order.is_some() {
                return Err(meta.error("`order` foi informado mais de uma vez"));
            }

            let value = meta.value()?;
            let literal: LitInt = value.parse()?;
            order = Some(literal.base10_parse::<u32>()?);
            Ok(())
        })?;
    }

    if !found_present_attribute {
        return Err(Error::new(
            field.span(),
            "campo sem `#[present(order = N)]`",
        ));
    }

    order.ok_or_else(|| {
        Error::new(
            field.span(),
            "atributo `present` precisa declarar `order = N`",
        )
    })
}
