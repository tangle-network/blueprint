use proc_macro2::Ident;
use quote::quote;
use syn::DeriveInput;

use crate::{cfg::FieldInfo, crate_path::CratePath};

/// Generate the `EVMProviderContext` implementation for the given struct.
pub fn generate_context_impl(
    DeriveInput {
        ident: name,
        generics,
        ..
    }: DeriveInput,
    config_field: FieldInfo,
    sdk: &CratePath,
) -> proc_macro2::TokenStream {
    let field_access = match config_field {
        FieldInfo::Named(ident) => quote! { self.#ident },
        FieldInfo::Unnamed(index) => quote! { self.#index },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let network_ty_ident = Ident::new(&format!("__{}Network", name), name.span());
    let provider_ty_ident = Ident::new(&format!("__{}Provider", name), name.span());
    let sdk = sdk.tokens();

    quote! {
        type #network_ty_ident = #sdk::alloy::network::Ethereum;
        type #provider_ty_ident = #sdk::alloy::providers::fillers::FillProvider<
            #sdk::alloy::providers::fillers::JoinFill<
                #sdk::alloy::providers::Identity,
                <#network_ty_ident as #sdk::alloy::providers::fillers::RecommendedFillers>::RecommendedFillers,
            >,
            #sdk::alloy::providers::RootProvider,
            #network_ty_ident,
        >;

        #[automatically_derived]
        impl #impl_generics #sdk::contexts::instrumented_evm_client::EvmInstrumentedClientContext for #name #ty_generics #where_clause {
            async fn evm_client(&self) -> #sdk::contexts::instrumented_evm_client::InstrumentedClient {
                #sdk::contexts::instrumented_evm_client::InstrumentedClient::new(
                    #field_access.http_rpc_endpoint.clone(),
                ).await.expect("Failed to create EVM client")
            }
        }
    }
}
