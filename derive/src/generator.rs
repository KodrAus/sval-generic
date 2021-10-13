use crate::{attr, bound};
use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, Type};

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

    let field_ident = fields
        .named
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();
    let field_str = fields
        .named
        .iter()
        .map(attr::name_of_field)
        .collect::<Vec<_>>();
    let field_ty = fields.named.iter().map(|f| &f.ty).collect::<Vec<_>>();

    let field_variant = field_ident
        .iter()
        .map(|f| Ident::new(&format!("Field_{}", f.to_string()), Span::call_site()))
        .collect::<Vec<_>>();

    let begin_variant = Ident::new("Begin", Span::call_site());

    let end_variant = Ident::new("End", Span::call_site());

    let field_transition = field_variant
        .iter()
        .map(|f| quote!(__GeneratorState::#f { generator: None }))
        .chain(Some(quote!(__GeneratorState::#end_variant)))
        .collect::<Vec<_>>();

    let field_transition_first = &field_transition[0];
    let field_transition_next = &field_transition[1..];

    let num_fields = field_ident.len();

    TokenStream::from(quote! {
        #[allow(non_upper_case_globals, dead_code, unused_mut)]
        const _: () = {
            extern crate sval_generic_api;

            impl sval_generic_api::generator::GeneratorValue for #ident {
                type Generator<'a> = __Generator<'a>;

                fn generator<'a>(&'a self) -> Self::Generator<'a> {
                    __Generator {
                        value: self,
                        generator: __GeneratorState::Begin,
                    }
                }

                fn stream<'a, R: sval_generic_api::Receiver<'a>>(&'a self, mut receiver: R) -> sval_generic_api::Result {
                    receiver.type_tagged_map_begin(sval_generic_api::tag::type_tag(#tag), Some(#num_fields))?;

                    #(
                        receiver.map_field_entry(#field_str, sval_generic_api::generator::GeneratorValue::as_value(&self.#field_ident))?;
                    )*

                    receiver.type_tagged_map_end()
                }
            }

            pub struct __Generator<'a> {
                value: &'a #ident,
                generator: __GeneratorState<'a>,
            }

            pub enum __GeneratorState<'a> {
                #begin_variant,
                #(
                    #field_variant {
                        generator: Option<<#field_ty as sval_generic_api::generator::GeneratorValue>::Generator<'a>>,
                    },
                )*
                #end_variant,
                Done,
            }

            impl<'a> sval_generic_api::generator::GeneratorImpl<'a> for __Generator<'a> {
                const MAY_YIELD: bool = true;

                fn resume<R: sval_generic_api::Receiver<'a>>(
                    &mut self,
                    receiver: &mut R,
                ) -> sval_generic_api::Result<sval_generic_api::generator::GeneratorState> {
                    match self.generator {
                        __GeneratorState::#begin_variant => {
                            receiver.type_tagged_map_begin(sval_generic_api::tag::type_tag("MyType"), Some(#num_fields))?;

                            self.generator = #field_transition_first;

                            Ok(sval_generic_api::generator::GeneratorState::Yield)
                        },

                        #(
                            __GeneratorState::#field_variant { ref mut generator } => {
                                if !<<#field_ty as sval_generic_api::generator::GeneratorValue>::Generator<'a>>::MAY_YIELD {
                                    receiver.map_field_entry(#field_str, sval_generic_api::generator::GeneratorValue::as_value(&self.value.#field_ident))?;
                                }
                                else {
                                    match generator {
                                        None => {
                                            receiver.map_field(#field_str)?;
                                            receiver.map_value_begin()?;

                                            let mut generator_slot = generator;
                                            let mut generator = sval_generic_api::generator::GeneratorValue::generator(&self.value.#field_ident);

                                            if let sval_generic_api::generator::GeneratorState::Yield = generator.resume(receiver)? {
                                                *generator_slot = Some(generator);
                                                return Ok(sval_generic_api::generator::GeneratorState::Yield);
                                            }
                                        }
                                        Some(generator) => {
                                            if let sval_generic_api::generator::GeneratorState::Yield = generator.resume(receiver)? {
                                                return Ok(sval_generic_api::generator::GeneratorState::Yield);
                                            }
                                        }
                                    }

                                    receiver.map_value_end()?;
                                }

                                self.generator = #field_transition_next;

                                Ok(sval_generic_api::generator::GeneratorState::Yield)
                            },
                        )*

                        __GeneratorState::#end_variant => {
                            receiver.type_tagged_map_end()?;

                            self.generator = __GeneratorState::Done;

                            Ok(sval_generic_api::generator::GeneratorState::Done)
                        }

                        __GeneratorState::Done => Ok(sval_generic_api::generator::GeneratorState::Done),
                    }
                }
            }
        };
    })
}
