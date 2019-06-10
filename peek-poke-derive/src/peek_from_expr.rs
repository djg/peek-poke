use crate::{max_size_expr, peek_poke::Generate};
use proc_macro2::TokenStream;
use quote::quote;
use std::str::FromStr;
use syn::{DataEnum, DataStruct, Fields, Ident, Index};

/// Calculates serialize expression for fields
fn get_peek_from_expr_for_fields(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Unit => quote! {},
        Fields::Named(named_fields) => {
            if named_fields.named.is_empty() {
                quote! {}
            } else {
                let mut exprs = Vec::with_capacity(named_fields.named.len());

                for field in named_fields.named.iter() {
                    let field_name = match &field.ident {
                        None => unreachable!(),
                        Some(ref ident) => quote! { #ident },
                    };

                    let field_ref =
                        TokenStream::from_str(&format!("&mut (*output).{}", field_name)).unwrap();
                    let field_ty = &field.ty;

                    exprs.push(quote! {
                        let bytes = <#field_ty>::peek_from(bytes, #field_ref);
                    });
                }

                quote! {
                    #(#exprs;)*
                }
            }
        }
        Fields::Unnamed(unnamed_fields) => {
            if unnamed_fields.unnamed.is_empty() {
                quote! {}
            } else {
                let mut exprs = Vec::with_capacity(unnamed_fields.unnamed.len());

                for (n, field) in unnamed_fields.unnamed.iter().enumerate() {
                    let field_ref =
                        TokenStream::from_str(&format!("&mut (*output).{}", n)).unwrap();
                    let field_ty = &field.ty;

                    exprs.push(quote! {
                        let bytes = <#field_ty>::peek_from(bytes, #field_ref);
                    });
                }

                quote! {
                    #(#exprs)*
                }
            }
        }
    }
}

fn get_peek_from_uninit_expr_for_fields(
    fields: &Fields,
    gen: Generate,
) -> (TokenStream, TokenStream) {
    match fields {
        Fields::Unit => (quote! {}, quote! {}),
        Fields::Named(named_fields) => {
            if named_fields.named.is_empty() {
                (quote! {}, quote! {})
            } else {
                let mut exprs = Vec::with_capacity(named_fields.named.len());
                let mut fields = Vec::with_capacity(named_fields.named.len());

                for field in &named_fields.named {
                    let field_name = match &field.ident {
                        None => unreachable!(),
                        Some(ref ident) => quote! { #ident },
                    };

                    let init = if gen == Generate::PeekDefault {
                        quote! {
                            let (#field_name, bytes) = peek_poke::peek_from_default(bytes);

                        }
                    } else {
                        quote! {
                            let (#field_name, bytes) = peek_poke::peek_from_uninit(bytes);
                        }
                    };
                    exprs.push(init);
                    fields.push(field_name);
                }
                (
                    quote! {
                        #(#exprs)*
                    },
                    quote! {
                        #(#fields),*
                    },
                )
            }
        }
        Fields::Unnamed(unnamed_fields) => {
            if unnamed_fields.unnamed.is_empty() {
                (quote! {}, quote! {})
            } else {
                let mut exprs = Vec::with_capacity(unnamed_fields.unnamed.len());
                let mut fields = Vec::with_capacity(unnamed_fields.unnamed.len());

                for n in 0..unnamed_fields.unnamed.len() {
                    let field_name = TokenStream::from_str(&format!("_{}", n)).unwrap();

                    let init = if gen == Generate::PeekDefault {
                        quote! {
                            let (#field_name, bytes) = peek_poke::peek_from_default(bytes);
                        }
                    } else {
                        quote! {
                            let (#field_name, bytes) = peek_poke::peek_from_uninit(bytes);
                        }
                    };
                    exprs.push(init);
                    fields.push(field_name);
                }
                (
                    quote! {
                        #(#exprs)*
                    },
                    quote! {
                        #(#fields),*
                    },
                )
            }
        }
    }
}

/// Calculates size expression for [`DataStruct`](syn::DataStruct)
pub fn for_struct(struct_data: &DataStruct) -> TokenStream {
    let exprs = get_peek_from_expr_for_fields(&struct_data.fields);
    quote! {
        #exprs
        bytes
    }
}

/// Calculates size expression for [`DataEnum`](syn::DataEnum)
pub fn for_enum(name: &Ident, enum_data: &DataEnum, gen: Generate) -> TokenStream {
    let variant_count = enum_data.variants.len();

    let size_type = max_size_expr::get_variant_count_max_size_type(variant_count);
    let mut match_exprs = Vec::with_capacity(variant_count);

    let variant_expr = quote! {
        let (variant, bytes) = peek_poke::peek_from_default::<#size_type>(bytes);
    };

    for (i, variant) in enum_data.variants.iter().enumerate() {
        let variant_name = &variant.ident;

        let index = Index::from(i);
        let (init_expr, fields_expr) = get_peek_from_uninit_expr_for_fields(&variant.fields, gen);
        let self_assign_expr = match &variant.fields {
            Fields::Named(..) => quote! {
                *output = #name:: #variant_name { #fields_expr };
            },
            Fields::Unnamed(..) => quote! {
                *output = #name:: #variant_name(#fields_expr);
            },
            Fields::Unit => quote! {
                *output = #name:: #variant_name;
            },
        };

        match_exprs.push(quote! {
            #index => {
                #init_expr
                #self_assign_expr
                bytes
            }
        });
    }

    match_exprs.push(quote! {
        _ => unreachable!()
    });

    let match_expr = quote! {
        match variant {
            #(#match_exprs),*
        }
    };

    quote! {
        #variant_expr
        #match_expr
    }
}
