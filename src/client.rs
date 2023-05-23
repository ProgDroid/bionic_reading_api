use reqwest::Url;

use crate::{request::Builder as RequestBuilder, secret::Secret};
use std::str::FromStr;

pub const BASE_URL_DOMAIN: &str = "bionic-reading1.p.rapidapi.com";

const BASE_URL: &str = "https://bionic-reading1.p.rapidapi.com";
const CONVERT_PATH: &str = "convert";

#[must_use]
#[derive(Clone)]
pub struct Client {
    pub api_key: Secret,
    pub url: Url,
}

impl Client {
    pub fn new<T: Into<String>>(api_key: T) -> Self {
        Self {
            api_key: Secret::from_str(&api_key.into()).expect("Client::new()"),
            url: Url::parse(BASE_URL).expect("base url"),
        }
    }

    #[must_use]
    pub fn convert<T: Into<String>>(self, input: T) -> RequestBuilder {
        let mut url = self.url;
        url.set_path(CONVERT_PATH);

        RequestBuilder::new(
            Self {
                api_key: self.api_key,
                url,
            },
            input.into(),
        )
    }
}
