use crate::{attr, bound};
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident};

pub(crate) fn derive(input: DeriveInput) -> TokenStream {
    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields,
        _ => panic!("currently only structs with named fields are supported"),
    };

    let ident = input.ident;
    let tag = ident.to_string();
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();

    let field_ident = &fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let field_lit = fields.named.iter().map(attr::name_of_field);
    let field_id = fields.named.iter().enumerate().map(|(i, _)| i as u64);
    let num_fields = field_ident.len() as u64;

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&input.generics, bound);

    TokenStream::from(quote! {
        #[allow(non_upper_case_globals)]
        const _: () = {
            extern crate sval;

            impl #impl_generics sval::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'a, R: sval::Receiver<'a>>(&'a self, mut receiver: R) -> sval::Result {
                    receiver.tagged_begin(sval::data::tag().for_struct().with_label(#tag))?;
                    receiver.map_begin(Some(#num_fields))?;

                    #(
                        receiver.map_entry(
                            sval::data::tag()
                                .for_struct_field()
                                .with_label(#field_lit)
                                .with_id(#field_id),
                            &self.#field_ident
                        )?;
                    )*

                    receiver.map_end()?;
                    receiver.tagged_end(sval::data::tag().for_struct().with_label(#tag))
                }
            }
        };
    })
}
