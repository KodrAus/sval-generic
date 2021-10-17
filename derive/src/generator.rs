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

    let generator_ident = Ident::new(&format!("Generator_{}", ident.to_string()), Span::call_site());
    let generator_state_ident = Ident::new(&format!("GeneratorState_{}", ident.to_string()), Span::call_site());

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
        .map(|f| quote!(#generator_state_ident::#f { generator: None }))
        .chain(Some(quote!(#generator_state_ident::#end_variant)))
        .collect::<Vec<_>>();

    let field_transition_first = &field_transition[0];
    let field_transition_next = &field_transition[1..];

    let num_fields = field_ident.len();

    TokenStream::from(quote! {
        #[allow(non_upper_case_globals, dead_code, unused_mut)]
        const _: () = {
            extern crate sval_generic_api;

            impl sval_generic_api::generator::GeneratorValue for #ident {
                type Generator<'a> = #generator_ident<'a>;

                #[inline]
                fn generator<'a>(&'a self) -> Self::Generator<'a> {
                    #generator_ident {
                        value: self,
                        generator: #generator_state_ident::Begin,
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

            pub struct #generator_ident<'a> {
                value: &'a #ident,
                generator: #generator_state_ident<'a>,
            }

            pub enum #generator_state_ident<'a> {
                #begin_variant,
                #(
                    #field_variant {
                        generator: Option<<#field_ty as sval_generic_api::generator::GeneratorValue>::Generator<'a>>,
                    },
                )*
                #end_variant,
                Done,
            }

            impl<'a> sval_generic_api::generator::GeneratorImpl<'a> for #generator_ident<'a> {
                const MAY_YIELD: bool = true;

                #[inline]
                fn resume<R: sval_generic_api::Receiver<'a>>(
                    &mut self,
                    receiver: &mut R,
                ) -> sval_generic_api::Result<sval_generic_api::generator::GeneratorState> {
                    match self.generator {
                        #generator_state_ident::#begin_variant => {
                            receiver.type_tagged_map_begin(sval_generic_api::tag::type_tag(#tag), Some(#num_fields))?;

                            self.generator = #field_transition_first;

                            Ok(sval_generic_api::generator::GeneratorState::Yield)
                        },

                        #(
                            #generator_state_ident::#field_variant { ref mut generator } => {
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

                        #generator_state_ident::#end_variant => {
                            receiver.type_tagged_map_end()?;

                            self.generator = #generator_state_ident::Done;

                            Ok(sval_generic_api::generator::GeneratorState::Done)
                        }

                        #generator_state_ident::Done => Ok(sval_generic_api::generator::GeneratorState::Done),
                    }
                }
            }
        };
    })
}
