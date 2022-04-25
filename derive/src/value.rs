use crate::{attr, bound};
use proc_macro::TokenStream;
use syn::{
    spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed,
    Generics, Ident, Variant,
};

pub(crate) fn derive(input: DeriveInput) -> TokenStream {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(ref fields),
            ..
        }) => derive_struct(&input.ident, &input.generics, fields),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(ref fields),
            ..
        }) if fields.unnamed.len() == 1 => derive_newtype(&input.ident, &input.generics),
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(ref fields),
            ..
        }) => derive_tuple(&input.ident, &input.generics, fields),
        Data::Enum(DataEnum { variants, .. }) => {
            derive_enum(&input.ident, &input.generics, variants.iter())
        }
        _ => panic!("unimplemented"),
    }
}

fn derive_struct<'a>(ident: &Ident, generics: &Generics, fields: &FieldsNamed) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let match_arm = stream_struct(quote!(#ident), &ident, None, fields);

    TokenStream::from(quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval>>(&'sval self, mut stream: S) -> sval::Result {
                    match self {
                        #match_arm
                    }

                    Ok(())
                }
            }
        };
    })
}

fn derive_newtype<'a>(ident: &Ident, generics: &Generics) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let match_arm = stream_newtype(quote!(#ident), &ident, None);

    TokenStream::from(quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval>>(&'sval self, mut stream: S) -> sval::Result {
                    match self {
                        #match_arm
                    }

                    Ok(())
                }

                fn is_dynamic(&self) -> bool {
                    false
                }
            }
        };
    })
}

fn derive_tuple<'a>(ident: &Ident, generics: &Generics, fields: &FieldsUnnamed) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let match_arm = stream_tuple(quote!(#ident), &ident, None, fields);

    TokenStream::from(quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval>>(&'sval self, mut stream: S) -> sval::Result {
                    match self {
                        #match_arm
                    }

                    Ok(())
                }

                fn is_dynamic(&self) -> bool {
                    false
                }
            }
        };
    })
}

fn derive_enum<'a>(
    ident: &Ident,
    generics: &Generics,
    variants: impl Iterator<Item = &'a Variant> + 'a,
) -> TokenStream {
    let tag = ident.to_string();
    let tag = quote!(sval::Tag::Named { name: #tag, id: None });

    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let mut variant_match_arms = Vec::new();

    for variant in variants {
        let variant_ident = &variant.ident;

        variant_match_arms.push(match variant.fields {
            Fields::Named(ref fields) => stream_struct(
                quote!(#ident :: #variant_ident),
                &variant.ident,
                Some(variant_match_arms.len() as u128),
                fields,
            ),
            Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => stream_newtype(
                quote!(#ident :: #variant_ident),
                &variant.ident,
                Some(variant_match_arms.len() as u128),
            ),
            Fields::Unnamed(ref fields) => stream_tuple(
                quote!(#ident :: #variant_ident),
                &variant.ident,
                Some(variant_match_arms.len() as u128),
                fields,
            ),
            Fields::Unit => stream_constant(
                quote!(#ident :: #variant_ident),
                &variant.ident,
                Some(variant_match_arms.len() as u128),
            ),
        });
    }

    TokenStream::from(quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval>>(&'sval self, mut stream: S) -> sval::Result {
                    stream.enum_begin(Some(#tag))?;

                    match self {
                        #(#variant_match_arms)*
                    }

                    stream.enum_end()
                }

                fn is_dynamic(&self) -> bool {
                    false
                }
            }
        };
    })
}

fn stream_struct(
    path: proc_macro2::TokenStream,
    label: &Ident,
    id: Option<u128>,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    let tag = label.to_string();
    let id = match id {
        Some(id) => quote!(Some(#id)),
        None => quote!(None),
    };
    let tag = quote!(sval::Tag::Named { name: #tag, id: #id });

    let mut field_ident = Vec::new();
    let mut field_lit = Vec::new();
    let mut field_id = Vec::new();
    let mut field_count = 0usize;

    for field in &fields.named {
        field_ident.push(&field.ident);
        field_lit.push(attr::name_of_field(field));
        field_id.push(field_count as u128);

        field_count += 1;
    }

    quote!(#path { #(ref #field_ident,)* } => {
        stream.record_begin(Some(#tag), Some(#field_count))?;

        #(
            stream.record_value_begin(sval::TagNamed { name: #field_lit, id: Some(#field_id) })?;
            stream.value(#field_ident)?;
            stream.record_value_end()?;
        )*

        stream.record_end()?;
    })
}

fn stream_newtype(
    path: proc_macro2::TokenStream,
    label: &Ident,
    id: Option<u128>,
) -> proc_macro2::TokenStream {
    let tag = label.to_string();
    let id = match id {
        Some(id) => quote!(Some(#id)),
        None => quote!(None),
    };
    let tag = quote!(sval::Tag::Named { name: #tag, id: #id });

    quote!(#path(ref field0) => {
        stream.tagged_begin(Some(#tag))?;
        stream.value(field0)?;
        stream.tagged_end()?;
    })
}

fn stream_tuple(
    path: proc_macro2::TokenStream,
    label: &Ident,
    id: Option<u128>,
    fields: &FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let tag = label.to_string();
    let id = match id {
        Some(id) => quote!(Some(#id)),
        None => quote!(None),
    };
    let tag = quote!(sval::Tag::Named { name: #tag, id: #id });

    let mut field_ident = Vec::new();
    let mut field_id = Vec::new();
    let mut field_count = 0usize;

    for field in &fields.unnamed {
        field_ident.push(Ident::new(&format!("field{}", field_count), field.span()));
        field_id.push(field_count as u128);
        field_count += 1;
    }

    quote!(#path(#(ref #field_ident,)*) => {
        stream.tuple_begin(Some(#tag), Some(#field_count))?;

        #(
            stream.tuple_value_begin(sval::TagUnnamed { id: #field_id })?;
            stream.value(#field_ident)?;
            stream.tuple_value_end()?;
        )*

        stream.tuple_end()?;
    })
}

fn stream_constant(
    path: proc_macro2::TokenStream,
    label: &Ident,
    id: Option<u128>,
) -> proc_macro2::TokenStream {
    let constant = label.to_string();
    let id = match id {
        Some(id) => quote!(Some(#id)),
        None => quote!(None),
    };
    let tag = quote!(sval::Tag::Named { name: #constant, id: #id });

    quote!(#path => {
        stream.constant_begin(Some(#tag))?;
        stream.value(#constant)?;
        stream.constant_end()?;
    })
}
