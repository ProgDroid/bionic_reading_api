//! Bionic Reading parameters and result wrapper
//!
//! Parameters allow customising the highlights in the resulting string.
//!
//! ## Example
//!
//! ```rust,no_run
//! use bionic_reading_api::{
//!     bionic::{Fixation, Saccade},
//!     client::Client,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let res = Client::new("api_key")
//!         .convert("Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua.")
//!         .fixation(Fixation::Strong)
//!         .saccade(Saccade::Fewest)
//!         .send()
//!         .await?;
//!
//!     let html = res.html().unwrap();
//!     println!("{html}");
//!
//!     Ok(())
//! }
//! ```

use reqwest::Error as ReqwestError;
use std::{convert::From, fmt::Debug};

/// Custom API Errors
#[derive(thiserror::Error)]
pub enum Error {
    /// Failed to convert given text
    #[error("Failed to convert given text")]
    FailedToConvert(#[from] ReqwestError),
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{self}\n")?;
        let mut current = std::error::Error::source(&self);
        while let Some(cause) = current {
            writeln!(f, "Caused by:\n\t{cause}")?;
            current = cause.source();
        }
        Ok(())
    }
}

/// Fixation levels
///
/// Defines the [expression of the letter combinations](https://bionic-reading.com/br-method/).
///
/// Weakest has least amount of highlighted characters, strongest has the most.
pub enum Fixation {
    Weakest,
    Weak,
    Average,
    Strong,
    Strongest,
}

impl Default for Fixation {
    fn default() -> Self {
        Self::Weakest
    }
}

impl From<Fixation> for u8 {
    fn from(value: Fixation) -> Self {
        match value {
            Fixation::Weakest => 1,
            Fixation::Weak => 2,
            Fixation::Average => 3,
            Fixation::Strong => 4,
            Fixation::Strongest => 5,
        }
    }
}

impl From<u8> for Fixation {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Weakest,
            2 => Self::Weak,
            4 => Self::Strong,
            5 => Self::Strongest,
            _ => Self::Average,
        }
    }
}

/// Saccade levels
///
/// Defines the [visual jumps from Fixation to Fixation](https://bionic-reading.com/br-method/).
///
/// Fewest saccades for biggest jumps, most sacccades for shortest jumps.
pub enum Saccade {
    Fewest,
    Few,
    Average,
    More,
    Most,
}

impl Default for Saccade {
    fn default() -> Self {
        Self::Fewest
    }
}

impl From<Saccade> for u8 {
    fn from(value: Saccade) -> Self {
        match value {
            Saccade::Fewest => 10,
            Saccade::Few => 20,
            Saccade::Average => 30,
            Saccade::More => 40,
            Saccade::Most => 50,
        }
    }
}

impl From<u8> for Saccade {
    fn from(value: u8) -> Self {
        match value {
            10 => Self::Fewest,
            20 => Self::Few,
            40 => Self::More,
            50 => Self::Most,
            _ => Self::Average,
        }
    }
}

/// Bionic Reading converted text.
///
/// Can be used in its raw form (as HTML) or Markdown.
#[cfg_attr(feature = "doc-tests", visible::StructFields(pub))]
pub struct Text {
    pub(crate) html: Option<String>,
}

impl Text {
    /// Get content as HTML.
    ///
    /// Once you have the response:
    /// ```rust,no_run
    /// # use bionic_reading_api::{
    /// #     bionic::{Fixation, Saccade},
    /// #     client::Client,
    /// #  };
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let response = Client::new("api_key")
    ///     .convert("Lorem ipsum dolor sit amet")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    /// Then it can be turned into HTML.
    /// ```rust
    /// # use bionic_reading_api::bionic::Text;
    /// # let html = Some(String::from("<b>Lor</b>em <b>ips</b>um <b>dol</b>or <b>si</b>t <b>ame</b>t"));
    /// # let response = Text { html };
    /// assert_eq!(response.html().unwrap(), "<b>Lor</b>em <b>ips</b>um <b>dol</b>or <b>si</b>t <b>ame</b>t");
    /// ```
    #[must_use]
    pub fn html(&self) -> Option<String> {
        self.html.as_ref().cloned()
    }

    /// Get content as Markdown, converted from HTML.
    ///
    /// Once you have the response:
    /// ```rust,no_run
    /// # use bionic_reading_api::{
    /// #     bionic::{Fixation, Saccade},
    /// #     client::Client,
    /// #  };
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let response = Client::new("api_key")
    ///     .convert("Lorem ipsum dolor sit amet")
    ///     .send()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    /// Then it can be turned into Markdown.
    /// ```rust
    /// # use bionic_reading_api::bionic::Text;
    /// # let html = Some(String::from("<b>Lor</b>em <b>ips</b>um <b>dol</b>or <b>si</b>t <b>ame</b>t"));
    /// # let response = Text { html };
    /// assert_eq!(response.markdown().unwrap(), "**Lor**em **ips**um **dol**or **si**t **ame**t");
    /// ```
    #[must_use]
    pub fn markdown(&self) -> Option<String> {
        self.html.as_ref().map(|str| html2md::parse_html(str))
    }
}
