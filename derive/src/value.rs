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
    let (impl_generics, ty_generics, _) = input.generics.split_for_impl();

    let dummy = Ident::new(
        &format!("_IMPL_SVAL_GENERIC_VALUE_FOR_{}", ident),
        Span::call_site(),
    );

    let fieldname = &fields.named.iter().map(|f| &f.ident).collect::<Vec<_>>();
    let fieldstr = fields.named.iter().map(attr::name_of_field);
    let num_fields = fieldname.len();

    let bound = parse_quote!(sval_generic_api::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&input.generics, bound);

    TokenStream::from(quote! {
        #[allow(non_upper_case_globals)]
        const #dummy: () = {
            extern crate sval_generic_api;

            impl #impl_generics sval_generic_api::Value for #ident #ty_generics #bounded_where_clause {
                fn stream<'a, S>(&'a self, mut stream: S) -> sval_generic_api::Result
                where
                    S: sval_generic_api::Stream<'a>
                {
                    stream.map_begin(Some(#num_fields))?;

                    #(
                        stream.map_field(#fieldstr, &self.#fieldname)?;
                    )*

                    stream.map_end()
                }
            }
        };
    })
}
