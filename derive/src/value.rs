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
                fn stream<'a, R: sval::Receiver<'a>>(&'a self, mut receiver: R) -> sval::Result {
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
                fn stream<'a, R: sval::Receiver<'a>>(&'a self, mut receiver: R) -> sval::Result {
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
                fn stream<'a, R: sval::Receiver<'a>>(&'a self, mut receiver: R) -> sval::Result {
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
            Fields::Unit => stream_enum_constant(
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
                fn stream<'a, R: sval::Receiver<'a>>(&'a self, mut receiver: R) -> sval::Result {
                    receiver.tagged_begin(sval::data::tag().for_enum().with_label(#tag))?;

                    match self {
                        #(#variant_match_arms)*
                    }

                    receiver.tagged_end(sval::data::tag().for_enum().with_label(#tag))
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
        receiver.tagged_begin(sval::data::tag().for_struct().with_label(#tag).with_id(#id))?;
        receiver.map_begin(Some(#field_count))?;

        #(
            receiver.map_entry(
                sval::data::tag()
                    .for_struct_field()
                    .with_label(#field_lit)
                    .with_id(#field_id)
                    .with_value(#field_lit),
                #field_ident,
            )?;
        )*

        receiver.map_end()?;
        receiver.tagged_end(sval::data::tag().for_struct().with_label(#tag).with_id(#id))?;
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
        receiver.tagged(sval::data::tag().with_label(#tag).with_id(#id).with_value(field0))?;
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
    let mut field_count = 0usize;

    for field in &fields.unnamed {
        field_ident.push(Ident::new(&format!("field{}", field_count), field.span()));
        field_count += 1;
    }

    quote!(#path(#(ref #field_ident,)*) => {
        receiver.tagged_begin(sval::data::tag().for_tuple().with_label(#tag).with_id(#id))?;
        receiver.seq_begin(Some(#field_count))?;

        #(
            receiver.seq_elem(#field_ident)?;
        )*

        receiver.seq_end()?;
        receiver.tagged_end(sval::data::tag().for_tuple().with_label(#tag).with_id(#id))?;
    })
}

fn stream_enum_constant(
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
        receiver.tagged(sval::data::tag().for_enum_constant().with_label(#tag).with_id(#id).with_value(#tag))?;
    })
}
