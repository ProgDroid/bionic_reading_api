//! Client used to call endpoints
//!
//! Requires API Key from [Bionic Reading Rapid API](https://rapidapi.com/bionic-reading-bionic-reading-default/api/bionic-reading1)
//!
//! ## Example
//!
//! ```rust,no_run
//! # use bionic_reading_api::{
//! #     bionic::{Fixation, Saccade},
//! #     client::Client,
//! # };
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let res = Client::new("api_key")
//!     .convert("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.")
//!     .send()
//!     .await?;
//! #     Ok(())
//! # }
//! ```

use crate::{requests::convert::Builder as ConvertBuilder, secret::Secret};
use reqwest::Url;
use std::str::FromStr;

pub(crate) const BASE_URL_DOMAIN: &str = "bionic-reading1.p.rapidapi.com";

const BASE_URL: &str = "https://bionic-reading1.p.rapidapi.com";
const CONVERT_PATH: &str = "convert";

/// Client used to call endpoints
#[must_use]
#[derive(Clone)]
pub struct Client {
    pub(crate) api_key: Secret,
    pub(crate) url: Url,
}

impl Client {
    /// Create new Client with given [Bionic Reading Rapid API key](https://rapidapi.com/bionic-reading-bionic-reading-default/api/bionic-reading1)
    pub fn new<T: Into<String>>(api_key: T) -> Self {
        Self {
            api_key: Secret::from_str(&api_key.into()).expect("Client::new()"),
            url: Url::parse(BASE_URL).expect("base url"),
        }
    }

    /// Convert to Bionic Reading highlighted string
    ///
    /// Returned request builder has sensible defaults for conversion parameters [Fixation](crate::bionic::Fixation) and [Saccade](crate::bionic::Saccade)
    /// ```rust,no_run
    /// # use bionic_reading_api::client::Client;
    /// # let client = Client::new("api_key");
    /// client.convert("Lorem ipsum dolor sit amet");
    /// ```
    #[must_use]
    pub fn convert<T: Into<String>>(self, input: T) -> ConvertBuilder {
        let mut url = self.url;
        url.set_path(CONVERT_PATH);

        ConvertBuilder::new(
            Self {
                api_key: self.api_key,
                url,
            },
            input.into(),
        )
    }
}
