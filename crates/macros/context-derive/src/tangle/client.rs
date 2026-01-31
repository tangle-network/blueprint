use quote::quote;
use syn::DeriveInput;

use crate::cfg::FieldInfo;

/// Generate the `TangleClientContext` implementation for the given struct.
pub fn generate_context_impl(
    DeriveInput {
        ident: name,
        generics,
        ..
    }: DeriveInput,
    config_field: FieldInfo,
) -> proc_macro2::TokenStream {
    let field_access_config = match config_field {
        FieldInfo::Named(ident) => quote! { self.#ident },
        FieldInfo::Unnamed(index) => quote! { self.#index },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let config_ty = quote! { ::blueprint_sdk::contexts::tangle::TangleClient };
    let error_ty = quote! { ::blueprint_sdk::contexts::tangle::Error };

    quote! {
        impl #impl_generics ::blueprint_sdk::contexts::tangle::TangleClientContext for #name #ty_generics #where_clause {
            fn tangle_client(&self) -> impl ::core::future::Future<Output = ::core::result::Result<#config_ty, #error_ty>> + ::core::marker::Send {
                ::blueprint_sdk::contexts::tangle::TangleClientContext::tangle_client(&#field_access_config)
            }
        }
    }
}
