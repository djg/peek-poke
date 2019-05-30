use crate::{max_size_expr, peek_from_expr, poke_into_expr};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, Data::*, DeriveInput, GenericParam, Generics};

/// Returns `PeekPoke` trait implementation
pub fn get_impl(input: DeriveInput) -> TokenStream {
    let name = input.ident;
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

    let poke_generics = add_trait_bound(input.generics.clone(), quote!{ peek_poke::Poke });
    let (impl_generics, ty_generics, where_clause) = poke_generics.split_for_impl();

    let poke_impl = quote! {
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(unused)]
        unsafe impl #impl_generics peek_poke::Poke for #name #ty_generics #where_clause {
            #[inline(always)]
            fn max_size() -> usize {
                #max_size
            }

            #[inline(always)]
            unsafe fn poke_into(&self, bytes: *mut u8) -> *mut u8 {
                #poke_into
            }
        }
    };

    let peek_generics = add_trait_bound(input.generics.clone(), quote!{ peek_poke::Peek });
    let (impl_generics, ty_generics, where_clause) = peek_generics.split_for_impl();

    let peek_impl = quote! {
        #[automatically_derived]
        #[allow(unused_qualifications)]
        #[allow(unused)]
        impl #impl_generics peek_poke::Peek for #name #ty_generics #where_clause {
            #[inline(always)]
            unsafe fn peek_from(&mut self, bytes: *const u8) -> *const u8 {
                #peek_from
            }
        }
    };

    quote! {
        #poke_impl
        #peek_impl
    }
}

// Add a bound `T: PeekPoke` for every type parameter `T`.
fn add_trait_bound(mut generics: Generics, bound: impl ToTokens) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(#bound));
        }
    }
    generics
}
