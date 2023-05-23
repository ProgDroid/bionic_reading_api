use reqwest::Error as ReqwestError;
use std::{convert::From, fmt::Debug};

#[derive(thiserror::Error)]
pub enum Error {
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

pub struct Text {
    pub(crate) html: Option<String>,
}

impl Text {
    #[must_use]
    pub fn html(&self) -> Option<String> {
        self.html.as_ref().cloned()
    }

    #[must_use]
    pub fn markdown(&self) -> Option<String> {
        self.html.as_ref().map(|str| html2md::parse_html(str))
    }
}
