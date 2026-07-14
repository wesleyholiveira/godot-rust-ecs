//! Gera a composição ordenada de campos que implementam `Present<C>`.

use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, spanned::Spanned, Data, DeriveInput,
    Error, Field, Fields, LitInt, Result,
};

#[proc_macro_derive(PresentOutput, attributes(present))]
pub fn derive_present_output(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn expand(input: DeriveInput) -> Result<proc_macro2::TokenStream> {
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

    let mut ordered = Vec::with_capacity(fields.len());
    let mut used_orders: HashMap<u32, syn::Ident> = HashMap::new();

    for field in fields {
        let ident = field
            .ident
            .clone()
            .ok_or_else(|| Error::new(field.span(), "campo nomeado esperado"))?;
        let order = parse_order(&field)?;

        if let Some(previous) = used_orders.insert(order, ident.clone()) {
            return Err(Error::new(
                ident.span(),
                format!(
                    "ordem {order} duplicada entre `{previous}` e `{ident}`"
                ),
            ));
        }

        ordered.push((order, ident, field.ty));
    }

    ordered.sort_by_key(|(order, _, _)| *order);

    let field_idents: Vec<_> = ordered.iter().map(|(_, ident, _)| ident).collect();
    let field_types: Vec<_> = ordered.iter().map(|(_, _, ty)| ty).collect();

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
                &mut self,
                context: &mut __PresentationContext,
            ) {
                #(
                    <#field_types as crate::presentation::Present<__PresentationContext>>::present(
                        &mut self.#field_idents,
                        context,
                    );
                )*
            }
        }
    })
}

fn parse_order(field: &Field) -> Result<u32> {
    let mut order = None;
    let mut found = false;

    for attribute in field
        .attrs
        .iter()
        .filter(|attribute| attribute.path().is_ident("present"))
    {
        found = true;

        attribute.parse_nested_meta(|meta| {
            if !meta.path.is_ident("order") {
                return Err(meta.error("use somente `order = N`"));
            }

            if order.is_some() {
                return Err(meta.error("`order` foi informado mais de uma vez"));
            }

            let literal: LitInt = meta.value()?.parse()?;
            order = Some(literal.base10_parse::<u32>()?);
            Ok(())
        })?;
    }

    if !found {
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
