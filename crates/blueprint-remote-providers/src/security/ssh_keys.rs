use crate::{Error, Result, managed::ProviderErrorCode};

pub(crate) fn parse_public_key(value: &str) -> Result<ssh_key::PublicKey> {
    let value = value.trim();
    let unsupported = || Error::Provider(ProviderErrorCode::Unsupported);
    if value.contains(['\0', '\r', '\n']) {
        return Err(unsupported());
    }
    let mut key = ssh_key::PublicKey::from_openssh(value).map_err(|_| unsupported())?;
    key.set_comment("");
    Ok(key)
}

pub(crate) fn canonical_public_key(value: &str) -> Result<String> {
    parse_public_key(value)?
        .to_openssh()
        .map_err(|_| Error::Provider(ProviderErrorCode::Unsupported))
}
