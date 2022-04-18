use crate::{attr, bound};
use proc_macro::TokenStream;
use syn::spanned::Spanned;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Generics, Ident,
    Variant,
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
                Some(variant_match_arms.len() as u64),
                fields,
            ),
            Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => stream_newtype(
                quote!(#ident :: #variant_ident),
                &variant.ident,
                Some(variant_match_arms.len() as u64),
            ),
            Fields::Unnamed(ref fields) => stream_tuple(
                quote!(#ident :: #variant_ident),
                &variant.ident,
                Some(variant_match_arms.len() as u64),
                fields,
            ),
            Fields::Unit => stream_constant(
                quote!(#ident :: #variant_ident),
                &variant.ident,
                Some(variant_match_arms.len() as u64),
            ),
        });
    }

    TokenStream::from(quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval>>(&'sval self, mut stream: S) -> sval::Result {
                    stream.enum_begin(sval::tag().with_label(#tag))?;

                    match self {
                        #(#variant_match_arms)*
                    }

                    stream.enum_end()
                }
            }
        };
    })
}

fn stream_struct(
    path: proc_macro2::TokenStream,
    label: &Ident,
    id: Option<u64>,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    let tag = label.to_string();
    let id = match id {
        Some(id) => quote!(Some(#id)),
        None => quote!(None),
    };

    let mut field_ident = Vec::new();
    let mut field_lit = Vec::new();
    let mut field_id = Vec::new();
    let mut field_count = 0usize;

    for field in &fields.named {
        field_ident.push(&field.ident);
        field_lit.push(attr::name_of_field(field));
        field_id.push(field_count as u64);

        field_count += 1;
    }

    quote!(#path { #(ref #field_ident,)* } => {
        stream.struct_map_begin(sval::tag().with_label(#tag).with_id(#id), Some(#field_count))?;

        #(
            stream.struct_map_key_begin(sval::tag().with_label(#field_lit).with_id(#field_id))?;
            stream.value(#field_lit)?;
            stream.struct_map_key_end()?;

            stream.struct_map_value_begin(sval::tag().with_label(#field_lit).with_id(#field_id))?;
            stream.value(#field_ident)?;
            stream.struct_map_value_end()?;
        )*

        stream.struct_map_end()?;
    })
}

fn stream_newtype(
    path: proc_macro2::TokenStream,
    label: &Ident,
    id: Option<u64>,
) -> proc_macro2::TokenStream {
    let tag = label.to_string();
    let id = match id {
        Some(id) => quote!(Some(#id)),
        None => quote!(None),
    };

    quote!(#path(ref field0) => {
        stream.tagged_begin(sval::tag().with_label(#tag).with_id(#id))?;
        stream.value(field0)?;
        stream.tagged_end()?;
    })
}

fn stream_tuple(
    path: proc_macro2::TokenStream,
    label: &Ident,
    id: Option<u64>,
    fields: &FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let tag = label.to_string();
    let id = match id {
        Some(id) => quote!(Some(#id)),
        None => quote!(None),
    };

    let mut field_ident = Vec::new();
    let mut field_id = Vec::new();
    let mut field_count = 0usize;

    for field in &fields.unnamed {
        field_ident.push(Ident::new(&format!("field{}", field_count), field.span()));
        field_id.push(field_count as u64);
        field_count += 1;
    }

    quote!(#path(#(ref #field_ident,)*) => {
        stream.struct_seq_begin(sval::tag().with_label(#tag).with_id(#id), Some(#field_count))?;

        #(
            stream.struct_seq_value_begin(sval::tag().with_id(#field_id))?;
            stream.value(#field_ident)?;
            stream.struct_seq_value_end()?;
        )*

        stream.struct_seq_end()?;
    })
}

fn stream_constant(
    path: proc_macro2::TokenStream,
    label: &Ident,
    id: Option<u64>,
) -> proc_macro2::TokenStream {
    let tag = label.to_string();
    let id = match id {
        Some(id) => quote!(Some(#id)),
        None => quote!(None),
    };

    quote!(#path => {
        stream.constant_begin(sval::tag().with_label(#tag).with_id(#id))?;
        stream.value(#tag)?;
        stream.constant_end()?;
    })
}
