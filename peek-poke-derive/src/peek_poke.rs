use crate::{max_size_expr, peek_from_expr, poke_into_expr};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Data::*, DeriveInput, GenericParam, Generics};

/// Returns `PeekPoke` trait implementation
pub fn get_impl(input: DeriveInput) -> TokenStream {
    let name = input.ident;
    let generics = add_peek_poke_trait_bound(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let (max_size, poke_into, peek_from) = match &input.data {
        Struct(ref struct_data) => (
            max_size_expr::for_struct(&struct_data),
            poke_into_expr::for_struct(&name, &struct_data),
            peek_from_expr::for_struct(&struct_data),
        ),
        Enum(ref enum_data) => (
            max_size_expr::for_enum(&enum_data),
            poke_into_expr::for_enum(&name, &enum_data),
            peek_from_expr::for_enum(&name, &enum_data),
        ),
        Union(_) => panic!("This macro cannot be used on unions!"),
    };

    quote! {
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(unused)]
        impl #impl_generics PeekPoke for #name #ty_generics #where_clause {
            #[inline(always)]
            fn max_size() -> usize {
                #max_size
            }

            #[inline(always)]
            fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
                #poke_into
            }

            #[inline(always)]
            fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
                #peek_from
            }
        }
    }
}

// Add a bound `T: PeekPoke` for every type parameter `T`.
fn add_peek_poke_trait_bound(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(peek_poke::PeekPoke));
        }
    }
    generics
}
