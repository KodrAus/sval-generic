use crate::{attr, bound};
use proc_macro::TokenStream;
use syn::spanned::Spanned;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed, Generics,
    Ident, Variant,
};

pub(crate) fn derive(input: DeriveInput) -> TokenStream {
    match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => derive_struct(&input.ident, &input.generics, fields.named.iter()),
        Data::Enum(DataEnum { variants, .. }) => {
            derive_enum(&input.ident, &input.generics, variants.iter())
        }
        _ => panic!("unimplemented"),
    }
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
        variant_match_arms.push(match variant.fields {
            Fields::Named(ref fields) => {
                derive_enum_struct_variant(ident, variant, variant_match_arms.len() as u64, fields)
            }
            Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
                derive_enum_newtype_variant(ident, variant, variant_match_arms.len() as u64)
            }
            Fields::Unnamed(ref fields) => {
                derive_enum_tuple_variant(ident, variant, variant_match_arms.len() as u64, fields)
            }
            Fields::Unit => {
                derive_enum_const_variant(ident, variant, variant_match_arms.len() as u64)
            }
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

fn derive_enum_struct_variant(
    ident: &Ident,
    variant: &Variant,
    idx: u64,
    fields: &FieldsNamed,
) -> proc_macro2::TokenStream {
    let tag = variant.ident.to_string();
    let variant_ident = &variant.ident;

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

    quote!(#ident::#variant_ident { #(ref #field_ident,)* } => {
        receiver.tagged_begin(sval::data::tag().for_struct().with_label(#tag).with_id(#idx))?;
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
        receiver.tagged_end(sval::data::tag().for_struct().with_label(#tag).with_id(#idx))?;
    })
}

fn derive_enum_newtype_variant(
    ident: &Ident,
    variant: &Variant,
    idx: u64,
) -> proc_macro2::TokenStream {
    let tag = variant.ident.to_string();
    let variant_ident = &variant.ident;

    quote!(#ident::#variant_ident(ref field0) => {
        receiver.tagged(sval::data::tag().with_label(#tag).with_id(#idx).with_value(field0))?;
    })
}

fn derive_enum_tuple_variant(
    ident: &Ident,
    variant: &Variant,
    idx: u64,
    fields: &FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let tag = variant.ident.to_string();
    let variant_ident = &variant.ident;

    let mut field_ident = Vec::new();
    let mut field_count = 0usize;

    for field in &fields.unnamed {
        field_ident.push(Ident::new(&format!("field{}", field_count), field.span()));
        field_count += 1;
    }

    quote!(#ident::#variant_ident(#(ref #field_ident,)*) => {
        receiver.tagged_begin(sval::data::tag().for_tuple().with_label(#tag).with_id(#idx))?;
        receiver.seq_begin(Some(#field_count))?;

        #(
            receiver.seq_elem(#field_ident)?;
        )*

        receiver.seq_end()?;
        receiver.tagged_end(sval::data::tag().for_tuple().with_label(#tag).with_id(#idx))?;
    })
}

fn derive_enum_const_variant(
    ident: &Ident,
    variant: &Variant,
    idx: u64,
) -> proc_macro2::TokenStream {
    let tag = variant.ident.to_string();
    let variant_ident = &variant.ident;

    quote!(#ident::#variant_ident => {
        receiver.tagged(sval::data::tag().for_enum_constant().with_label(#tag).with_id(#idx).with_value(#tag))?;
    })
}

fn derive_struct<'a>(
    ident: &Ident,
    generics: &Generics,
    fields: impl Iterator<Item = &'a Field> + 'a,
) -> TokenStream {
    let tag = ident.to_string();
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    let mut field_ident = Vec::new();
    let mut field_lit = Vec::new();
    let mut field_id = Vec::new();
    let mut field_count = 0usize;

    for field in fields {
        field_ident.push(&field.ident);
        field_lit.push(attr::name_of_field(field));
        field_id.push(field_count as u64);

        field_count += 1;
    }

    TokenStream::from(quote! {
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'a, R: sval::Receiver<'a>>(&'a self, mut receiver: R) -> sval::Result {
                    receiver.tagged_begin(sval::data::tag().for_struct().with_label(#tag))?;
                    receiver.map_begin(Some(#field_count))?;

                    #(
                        receiver.map_entry(
                            sval::data::tag()
                                .for_struct_field()
                                .with_label(#field_lit)
                                .with_id(#field_id)
                                .with_value(#field_lit),
                            &self.#field_ident,
                        )?;
                    )*

                    receiver.map_end()?;
                    receiver.tagged_end(sval::data::tag().for_struct().with_label(#tag))
                }
            }
        };
    })
}
