use quote::quote;
use syn::DeriveInput;

use crate::{cfg::FieldInfo, crate_path::CratePath};

/// Generate the `KeystoreContext` implementation for the given struct.
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
    let sdk = sdk.tokens();

    quote! {
        impl #impl_generics #sdk::contexts::keystore::KeystoreContext for #name #ty_generics #where_clause {
            fn keystore(&self) -> #sdk::keystore::Keystore {
                <#sdk::runner::config::BlueprintEnvironment as #sdk::contexts::keystore::KeystoreContext>::keystore(&#field_access)
            }
        }
    }
}
