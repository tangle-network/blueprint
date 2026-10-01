use quote::quote;
use syn::DeriveInput;

use crate::{cfg::FieldInfo, crate_path::CratePath};

/// Generate the `TangleClientContext` implementation for the given struct.
pub fn generate_context_impl(
    DeriveInput {
        ident: name,
        generics,
        ..
    }: DeriveInput,
    config_field: FieldInfo,
    sdk: &CratePath,
) -> proc_macro2::TokenStream {
    let field_access_config = match config_field {
        FieldInfo::Named(ident) => quote! { self.#ident },
        FieldInfo::Unnamed(index) => quote! { self.#index },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let sdk = sdk.tokens();

    let config_ty = quote! { #sdk::contexts::tangle::TangleClient };
    let error_ty = quote! { #sdk::contexts::tangle::Error };

    quote! {
        impl #impl_generics #sdk::contexts::tangle::TangleClientContext for #name #ty_generics #where_clause {
            fn tangle_client(&self) -> impl ::core::future::Future<Output = ::core::result::Result<#config_ty, #error_ty>> + ::core::marker::Send {
                #sdk::contexts::tangle::TangleClientContext::tangle_client(&#field_access_config)
            }
        }
    }
}
