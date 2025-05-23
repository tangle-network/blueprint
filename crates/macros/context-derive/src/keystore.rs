use quote::quote;
use syn::DeriveInput;

use crate::cfg::FieldInfo;

/// Generate the `KeystoreContext` implementation for the given struct.
pub fn generate_context_impl(
    DeriveInput {
        ident: name,
        generics,
        ..
    }: DeriveInput,
    config_field: FieldInfo,
) -> proc_macro2::TokenStream {
    let field_access = match config_field {
        FieldInfo::Named(ident) => quote! { self.#ident },
        FieldInfo::Unnamed(index) => quote! { self.#index },
    };

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::blueprint_sdk::contexts::keystore::KeystoreContext for #name #ty_generics #where_clause {
            fn keystore(&self) -> ::blueprint_sdk::keystore::Keystore {
                <::blueprint_sdk::runner::config::BlueprintEnvironment as ::blueprint_sdk::contexts::keystore::KeystoreContext>::keystore(&#field_access)
            }
        }
    }
}
