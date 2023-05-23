use std::{
    convert::Infallible,
    fmt::{Debug, Formatter},
    str::FromStr,
};

#[derive(Clone)]
pub struct Secret(String);

impl Secret {
    pub fn secret(&self) -> &str {
        &self.0
    }
}

impl FromStr for Secret {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl Debug for Secret {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "REDACTED")
    }
}
