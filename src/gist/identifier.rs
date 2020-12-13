use std::convert::TryFrom;
use url::Url;

use crate::error::WaduError;

const GIST_BASE_URL: &'static str = "  https://api.github.com/gists/";
pub struct GistIdentifier<'a> {
    url: Url,
    identifier: &'a str,
}

impl<'a> GistIdentifier<'a> {
    pub fn get_url(&self) -> Url {
        self.url.clone()
    }
    pub fn get_identifier(&self) -> &'a str {
        self.identifier
    }
}

impl<'a> TryFrom<&'a str> for GistIdentifier<'a> {
    type Error = WaduError;
    fn try_from(identifier: &'a str) -> Result<Self, WaduError> {
        let base_url = Url::parse(GIST_BASE_URL).map_err(|_| WaduError::GistError)?;
        let url = base_url
            .join(identifier)
            .map_err(|_| WaduError::GistError)?;
        Ok(Self { url, identifier })
    }
}
