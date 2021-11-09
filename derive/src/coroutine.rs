use crate::attr;
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

    let coroutine_begin_ident = Ident::new(
        &format!("Coroutine_{}_Begin", ident.to_string()),
        Span::call_site(),
    );
    let coroutine_end_ident = Ident::new(
        &format!("Coroutine_{}_End", ident.to_string()),
        Span::call_site(),
    );
    let coroutine_state_ident = Ident::new(
        &format!("CoroutineState_{}", ident.to_string()),
        Span::call_site(),
    );
    let coroutine_state_field_ident = Ident::new(
        &format!("CoroutineState_{}_Field", ident.to_string()),
        Span::call_site(),
    );

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

    let field_enter = field_ident
        .iter()
        .map(|f| Ident::new(&format!("enter_field_{}", f.to_string()), Span::call_site()))
        .collect::<Vec<_>>();

    let coroutine_state_field_variant = field_ident
        .iter()
        .map(|f| Ident::new(&format!("Field_{}", f.to_string()), Span::call_site()))
        .collect::<Vec<_>>();

    let coroutine_field_ident = field_ident
        .iter()
        .map(|f| {
            Ident::new(
                &format!("Coroutine_{}_Field_{}", ident.to_string(), f.to_string()),
                Span::call_site(),
            )
        })
        .collect::<Vec<_>>();

    let transition_begin = coroutine_field_ident
        .first()
        .unwrap_or(&coroutine_end_ident);

    let transition_field = coroutine_field_ident
        .iter()
        .skip(1)
        .chain(Some(&coroutine_end_ident))
        .collect::<Vec<_>>();

    let num_fields = field_ident.len();

    TokenStream::from(quote! {
        #[allow(non_upper_case_globals, non_camel_case_types, dead_code, unused_mut)]
        const _: () = {
            extern crate sval_generic_api_coroutine;

            impl sval_generic_api_coroutine::value::CoroutineValue for #ident {
                type State<'a> = #coroutine_state_ident<'a>;
                type Coroutine<'a, R: sval_generic_api_coroutine::Receiver<'a>> = #coroutine_begin_ident;

                #[inline]
                fn state<'a>(&'a self) -> Self::State<'a> {
                    #coroutine_state_ident {
                        value: self,
                        field: None,
                    }
                }
            }

            pub struct #coroutine_state_ident<'a> {
                value: &'a #ident,
                field: Option<#coroutine_state_field_ident<'a>>,
            }

            pub enum #coroutine_state_field_ident<'a> {
                #(
                    #coroutine_state_field_variant(sval_generic_api_coroutine::co::Coroutine<<#field_ty as sval_generic_api_coroutine::value::CoroutineValue>::State<'a>>),
                )*
            }

            impl<'a> #coroutine_state_ident<'a> {
                #(
                    fn #field_enter(self: std::pin::Pin<&mut Self>) -> std::pin::Pin<&mut sval_generic_api_coroutine::co::Coroutine<<#field_ty as sval_generic_api_coroutine::value::CoroutineValue>::State<'a>>> {
                        let self_mut = unsafe { self.get_unchecked_mut() };

                        self_mut.field = Some(#coroutine_state_field_ident::#coroutine_state_field_variant(sval_generic_api_coroutine::co::Coroutine::new(
                            sval_generic_api_coroutine::value::CoroutineValue::state(&self_mut.value.#field_ident),
                        )));

                        if let Some(#coroutine_state_field_ident::#coroutine_state_field_variant(ref mut slot)) = self_mut.field {
                            unsafe { std::pin::Pin::new_unchecked(slot) }
                        } else {
                            unreachable!()
                        }
                    }
                )*

                fn exit_field(self: std::pin::Pin<&mut Self>) -> bool {
                    unsafe { self.get_unchecked_mut() }.field.take().is_some()
                }
            }

            pub struct #coroutine_begin_ident;
            impl<'a, R: sval_generic_api_coroutine::Receiver<'a>> sval_generic_api_coroutine::co::Resume<'a, R> for #coroutine_begin_ident {
                type State = #coroutine_state_ident<'a>;

                const MAY_YIELD: bool = true;

                fn resume(mut cx: sval_generic_api_coroutine::co::Context<Self, R>) -> sval_generic_api_coroutine::Result<sval_generic_api_coroutine::co::Yield<Self>> {
                    cx.receiver().type_tagged_map_begin(sval_generic_api_coroutine::tag::type_tag(#tag), Some(#num_fields))?;

                    cx.yield_resume::<#transition_begin>()
                }
            }

            #(
                struct #coroutine_field_ident;
                impl<'a, R: sval_generic_api_coroutine::Receiver<'a>> sval_generic_api_coroutine::co::Resume<'a, R> for #coroutine_field_ident {
                    type State = #coroutine_state_ident<'a>;

                    fn resume(mut cx: sval_generic_api_coroutine::co::Context<Self, R>) -> sval_generic_api_coroutine::Result<sval_generic_api_coroutine::co::Yield<Self>> {
                        let (receiver, state) = cx.state();

                        if !<<#field_ty as sval_generic_api_coroutine::value::CoroutineValue>::Coroutine<'a, R> as sval_generic_api_coroutine::co::Resume<'a, R>>::MAY_YIELD {
                            receiver.map_field_entry(#field_str, &state.value.#field_ident)?;

                            cx.yield_resume::<#transition_field>()
                        }
                        else {
                            if state.exit_field() {
                                receiver.map_value_end()?;
                            }

                            receiver.map_field(#field_str)?;
                            receiver.map_value_begin()?;

                            cx.yield_into_resume::<<#field_ty as sval_generic_api_coroutine::value::CoroutineValue>::Coroutine<'a, R>, #transition_field>(|state| {
                                state.#field_enter()
                            })
                        }
                    }
                }
            )*

            struct #coroutine_end_ident;
            impl<'a, R: sval_generic_api_coroutine::Receiver<'a>> sval_generic_api_coroutine::co::Resume<'a, R> for #coroutine_end_ident {
                type State = #coroutine_state_ident<'a>;

                fn resume(mut cx: sval_generic_api_coroutine::co::Context<Self, R>) -> sval_generic_api_coroutine::Result<sval_generic_api_coroutine::co::Yield<Self>> {
                    let (receiver, state) = cx.state();

                    if state.exit_field() {
                        receiver.map_value_end()?;
                    }

                    receiver.type_tagged_map_end()?;

                    cx.yield_return()
                }
            }
        };
    })
}
