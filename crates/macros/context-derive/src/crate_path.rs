//! Resolution of the SDK crate path used by generated code.
//!
//! The context derive macros historically hardcoded `::blueprint_sdk`, which
//! only resolves when the SDK is imported under that exact name. A blueprint
//! that renames the dependency, or re-exports the SDK from its own facade
//! crate, has no way to point the generated code at it. A `#[context(crate =
//! "...")]` helper attribute selects an alternative path, defaulting to
//! `::blueprint_sdk` when absent.

use proc_macro2::TokenStream;
use syn::{Attribute, Error, LitStr, Result, parse2};

/// Helper attribute that overrides the SDK crate path.
pub const CRATE_ATTR_NAME: &str = "context";

/// The resolved SDK crate path, defaulting to `::blueprint_sdk`.
#[derive(Clone)]
pub struct CratePath(TokenStream);

impl CratePath {
    /// The path as tokens, suitable for interpolation into generated code.
    pub fn tokens(&self) -> &TokenStream {
        &self.0
    }
}

impl Default for CratePath {
    fn default() -> Self {
        Self(parse2(quote::quote!(::blueprint_sdk)).expect("default SDK path parses"))
    }
}

/// Parse `#[context(crate = "path::to::sdk")]` off the derive input.
///
/// Only the `crate` key is recognised; any other key is an error so a typo
/// surfaces at compile time instead of silently resolving to the default.
pub fn parse_crate_path(attrs: &[Attribute]) -> Result<CratePath> {
    // A second `#[context(...)]` silently winning or losing to the first is
    // exactly the kind of typo that must surface at compile time. The main
    // loop returns at the first matching attribute, so duplicates must be
    // detected before it runs.
    let mut context_attrs = attrs
        .iter()
        .filter(|attr| attr.path().is_ident(CRATE_ATTR_NAME));
    let _first = context_attrs.next();
    if let Some(duplicate) = context_attrs.next() {
        return Err(Error::new_spanned(
            duplicate,
            format!("duplicate `{CRATE_ATTR_NAME}` attribute: only one is allowed"),
        ));
    }

    for attr in attrs {
        if !attr.path().is_ident(CRATE_ATTR_NAME) {
            continue;
        }

        let mut path: Option<LitStr> = None;

        attr.parse_nested_meta(|meta| {
            if !meta.path.is_ident("crate") {
                let key = meta
                    .path
                    .get_ident()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| "?".to_owned());
                return Err(meta.error(format!(
                    "unsupported `{CRATE_ATTR_NAME}` key `{key}`, expected `crate`"
                )));
            }

            if path.is_some() {
                return Err(meta.error("duplicate `crate` key"));
            }

            let value: LitStr = meta.value()?.parse()?;
            path = Some(value);
            Ok(())
        })?;

        let Some(lit) = path else {
            return Err(Error::new_spanned(
                attr,
                format!("`#[{CRATE_ATTR_NAME}(...)]` requires a `crate = \"...\"` key"),
            ));
        };

        // Validate eagerly: a path that does not parse would otherwise
        // surface as a confusing error at the use site in the user's crate.
        let parsed: TokenStream = lit.parse().map_err(|_| {
            Error::new_spanned(
                &lit,
                "expected a Rust path, e.g. `crate = \"my_crate::sdk\"`",
            )
        })?;

        if parsed.is_empty() {
            return Err(Error::new_spanned(
                &lit,
                "crate path must not be empty; omit the attribute to use `::blueprint_sdk`",
            ));
        }

        return Ok(CratePath(parsed));
    }

    Ok(CratePath::default())
}
