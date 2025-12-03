use quote::quote;
use syn::DeriveInput;

use crate::cfg::FieldInfo;

/// Generate the `TangleEvmClientContext` implementation for the given struct.
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

    let config_ty = quote! { ::blueprint_sdk::contexts::tangle_evm::TangleEvmClient };
    let error_ty = quote! { ::blueprint_sdk::contexts::tangle_evm::Error };

    quote! {
        impl #impl_generics ::blueprint_sdk::contexts::tangle_evm::TangleEvmClientContext for #name #ty_generics #where_clause {
            fn tangle_evm_client(&self) -> impl ::core::future::Future<Output = ::core::result::Result<#config_ty, #error_ty>> + ::core::marker::Send {
                ::blueprint_sdk::contexts::tangle_evm::TangleEvmClientContext::tangle_evm_client(&#field_access_config)
            }
        }
    }
}
