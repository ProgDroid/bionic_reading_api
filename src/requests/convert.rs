use crate::{
    bionic::{Error as BionicError, Fixation, Saccade, Text as BionicText},
    client::{self, Client as BionicClient},
};
use reqwest::{multipart::Form, Client as ReqwestClient};

const CONTENT_KEY: &str = "content";
const REQUEST_TYPE_KEY: &str = "request_type";
const REQUEST_TYPE: &str = "html";
const RESPONSE_TYPE_KEY: &str = "response_type";
const RESPONSE_TYPE: &str = "html";
const FIXATION_KEY: &str = "fixation";
const SACCADE_KEY: &str = "saccade";
const HEADER_RAPID_API_KEY: &str = "X-RapidAPI-Key";
const HEADER_RAPID_API_HOST: &str = "X-RapidAPI-Host";
const CONTENT_TYPE_KEY: &str = "content_type";
const CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

#[derive(Default)]
pub struct Request {
    input: String,
    fixation: Fixation,
    saccade: Saccade,
}

pub struct Builder {
    client: BionicClient,
    request: Request,
}

impl Builder {
    pub(crate) fn new(client: BionicClient, input: String) -> Self {
        Self {
            client,
            request: Request {
                input,
                ..Request::default()
            },
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn fixation(self, fixation: Fixation) -> Self {
        Self {
            request: Request {
                fixation,
                ..self.request
            },
            ..self
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn saccade(self, saccade: Saccade) -> Self {
        Self {
            request: Request {
                saccade,
                ..self.request
            },
            ..self
        }
    }

    pub async fn send(self) -> Result<BionicText, BionicError> {
        let client = ReqwestClient::new();

        let form = Form::new()
            .text(CONTENT_KEY, self.request.input)
            .text(REQUEST_TYPE_KEY, REQUEST_TYPE)
            .text(RESPONSE_TYPE_KEY, RESPONSE_TYPE)
            .text(FIXATION_KEY, u8::from(self.request.fixation).to_string())
            .text(SACCADE_KEY, u8::from(self.request.saccade).to_string());

        let url = self.client.url.clone();
        let domain = self.client.url.domain().unwrap_or(client::BASE_URL_DOMAIN);

        let res = client
            .post(url)
            .multipart(form)
            .header(HEADER_RAPID_API_KEY, self.client.api_key.secret())
            .header(HEADER_RAPID_API_HOST, domain)
            .header(CONTENT_TYPE_KEY, CONTENT_TYPE)
            .send()
            .await?;

        match res.error_for_status() {
            Ok(res) => Ok(BionicText {
                html: res.text().await.ok(),
            }),
            Err(err) => Err(BionicError::FailedToConvert(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_fixation() {
        let builder = Builder::new(BionicClient::new("api_key"), String::from("Input"));

        let builder = builder.fixation(Fixation::Strongest);

        let actual: u8 = builder.request.fixation.into();
        let expected: u8 = Fixation::Strongest.into();
        let default: u8 = Fixation::default().into();

        assert_ne!(
            actual, default,
            "Fixation was still default after being modified"
        );

        assert_eq!(actual, expected, "Fixation not set to given value");
    }

    #[test]
    fn test_set_saccade() {
        let builder = Builder::new(BionicClient::new("api_key"), String::from("Input"));

        let builder = builder.saccade(Saccade::Most);

        let actual: u8 = builder.request.saccade.into();
        let expected: u8 = Saccade::Most.into();
        let default: u8 = Saccade::default().into();

        assert_ne!(
            actual, default,
            "Saccade was still default after being modified"
        );

        assert_eq!(actual, expected, "Saccade not set to given value");
    }
}
