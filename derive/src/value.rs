use crate::{attr, bound};
use proc_macro::TokenStream;
use syn::{
    Data, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Generics, Ident, Variant,
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
    /*
    pub enum MyEnum {
        A,
        B { a: i32, b: i32 },
        C(i32),
        D(i32, i32),
    }

    impl Value for MyEnum {
        fn stream<'a, R: Receiver<'a>>(&'a self, mut receiver: R) -> Result {
            receiver.tagged_begin(data::tag().for_enum().with_label("MyEnum"))?;

            match self {
                MyEnum::A => receiver.tagged(
                    data::tag()
                        .for_enum_constant()
                        .with_label("A")
                        .with_value("A"),
                )?,
                MyEnum::B { ref a, ref b } => {
                    receiver.tagged_begin(data::tag().for_struct().with_label("B"))?;
                    receiver.map_begin(Some(2))?;

                    receiver.map_entry(
                        data::tag()
                            .for_struct_field()
                            .with_label("a")
                            .with_value("a"),
                        a,
                    )?;
                    receiver.map_entry(
                        data::tag()
                            .for_struct_field()
                            .with_label("b")
                            .with_value("b"),
                        b,
                    )?;

                    receiver.map_end()?;
                    receiver.tagged_end(data::tag().for_struct().with_label("B"))?;
                }
                MyEnum::C(ref v) => {
                    receiver.tagged(data::tag().with_label("C").with_value(v))?;
                }
                MyEnum::D(ref f0, ref f1) => {
                    receiver.tagged_begin(data::tag().for_struct().with_label("D"))?;
                    receiver.seq_begin(Some(2))?;

                    receiver.seq_elem(f0)?;
                    receiver.seq_elem(f1)?;

                    receiver.seq_end()?;
                    receiver.tagged_end(data::tag().for_struct().with_label("D"))?;
                }
            }

            receiver.tagged_end(data::tag().for_enum().with_label("MyEnum"))
        }
    }
    */
    unimplemented!()
}

fn derive_struct<'a>(
    ident: &Ident,
    generics: &Generics,
    fields: impl Iterator<Item = &'a Field> + 'a,
) -> TokenStream {
    let tag = ident.to_string();
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    let mut field_ident = Vec::new();
    let mut field_lit = Vec::new();
    let mut field_id = Vec::new();
    let mut field_count: usize = 0;

    for field in fields {
        field_ident.push(&field.ident);
        field_lit.push(attr::name_of_field(field));
        field_id.push(field_count as u64);

        field_count += 1;
    }

    let bound = parse_quote!(sval::Value);
    let bounded_where_clause = bound::where_clause_with_bound(&generics, bound);

    TokenStream::from(quote! {
        #[allow(non_upper_case_globals)]
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
