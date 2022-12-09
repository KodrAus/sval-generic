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
                fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
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

fn derive_newtype<'a>(ident: &Ident, generics: &Generics) -> TokenStream {
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let match_arm = stream_newtype(quote!(#ident), &ident, None);

    TokenStream::from(quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
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
                fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
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
    let (label, index) = label_index(ident, None);

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
                Some(variant_match_arms.len() as u32),
                fields,
            ),
            Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => stream_newtype(
                quote!(#ident :: #variant_ident),
                &variant.ident,
                Some(variant_match_arms.len() as u32),
            ),
            Fields::Unnamed(ref fields) => stream_tuple(
                quote!(#ident :: #variant_ident),
                &variant.ident,
                Some(variant_match_arms.len() as u32),
                fields,
            ),
            Fields::Unit => stream_constant(
                quote!(#ident :: #variant_ident),
                &variant.ident,
                Some(variant_match_arms.len() as u32),
            ),
        });
    }

    TokenStream::from(quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'sval, S: sval::Stream<'sval> + ?Sized>(&'sval self, stream: &mut S) -> sval::Result {
                    stream.enum_begin(None, #label, #index)?;

                    match self {
                        #(#variant_match_arms)*
                    }

                    stream.enum_end(None, #label, #index)
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
    index: Option<u32>,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    let (label, index) = label_index(label, index);

    let mut field_count = 0usize;
    let mut field_ident = Vec::new();
    let mut stream_field = Vec::new();

    for field in &fields.named {
        let label = attr::name_of_field(field);
        let label = quote!(sval::Label::new(#label));

        let ident = &field.ident;

        stream_field.push(if let Some(tag) = attr::tag(field) {
            quote!({
                stream.record_value_begin(#label)?;
                stream.tagged_begin(Some(sval::Tag::new(#tag)), None, None)?;
                sval::stream(stream, #ident)?;
                stream.tagged_end(Some(sval::Tag::new(#tag)), None, None)?;
                stream.record_value_end(#label)?;
            })
        } else {
            quote!({
                stream.record_value_begin(#label)?;
                sval::stream(stream, #ident)?;
                stream.record_value_end(#label)?;
            })
        });

        field_ident.push(ident.clone());
        field_count += 1;
    }

    quote!(#path { #(ref #field_ident,)* } => {
        stream.record_begin(None, #label, #index, Some(#field_count))?;

        #(
            #stream_field
        )*

        stream.record_end(None, #label, #index)?;
    })
}

fn stream_newtype(
    path: proc_macro2::TokenStream,
    label: &Ident,
    index: Option<u32>,
) -> proc_macro2::TokenStream {
    let (label, index) = label_index(label, index);

    quote!(#path(ref field0) => {
        stream.tagged_begin(None, #label, #index)?;
        sval::stream(stream, field0)?;
        stream.tagged_end(None, #label, #index)?;
    })
}

fn stream_tuple(
    path: proc_macro2::TokenStream,
    label: &Ident,
    index: Option<u32>,
    fields: &FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let (label, index) = label_index(label, index);

    let mut field_ident = Vec::new();
    let mut stream_field = Vec::new();
    let mut field_count = 0usize;

    for field in &fields.unnamed {
        let index = field_count as u32;

        let ident = Ident::new(&format!("field{}", field_count), field.span());

        stream_field.push(if let Some(tag) = attr::tag(field) {
            quote!({
                stream.tuple_value_begin(sval::Index::new(#index))?;
                stream.tagged_begin(Some(sval::Tag::new(#tag)), None, None)?;
                sval::stream(stream, #ident)?;
                stream.tagged_end(Some(sval::Tag::new(#tag)), None, None)?;
                stream.tuple_value_end(sval::Index::new(#index))?;
            })
        } else {
            quote!({
                stream.tuple_value_begin(sval::Index::new(#index))?;
                sval::stream(stream, #ident)?;
                stream.tuple_value_end(sval::Index::new(#index))?;
            })
        });

        field_ident.push(ident);
        field_count += 1;
    }

    quote!(#path(#(ref #field_ident,)*) => {
        stream.tuple_begin(None, #label, #index, Some(#field_count))?;

        #(
            #stream_field
        )*

        stream.tuple_end(None, #label, #index)?;
    })
}

fn stream_constant(
    path: proc_macro2::TokenStream,
    label: &Ident,
    id: Option<u32>,
) -> proc_macro2::TokenStream {
    let constant = label.to_string();
    let (label, index) = label_index(label, id);

    quote!(#path => {
        stream.constant_begin(None, #label, #index)?;
        sval::stream(stream, #constant)?;
        stream.constant_end(None, #label, #index)?;
    })
}

fn label_index(
    label: &Ident,
    index: Option<u32>,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let label = label.to_string();
    match index {
        Some(index) => (
            quote!(Some(sval::Label::new(#label))),
            quote!(Some(sval::Index::new(#index))),
        ),
        None => (quote!(Some(sval::Label::new(#label))), quote!(None)),
    }
}
