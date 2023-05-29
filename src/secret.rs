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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redacted() {
        let secret_string = "Some secret";
        let secret = Secret::from_str(secret_string);

        let log_attempt = format!("The secret is {secret:?}");

        assert!(
            !log_attempt.contains(secret_string),
            "Log attempt contains the secret string: `{log_attempt}`"
        );
        assert!(
            log_attempt.contains("REDACTED"),
            "Secret was not redacted in log attempt: `{log_attempt}`"
        );
    }
}
