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

    const SECRET_STRING: &str = "Some secret";

    #[test]
    fn test_redacted() {
        let secret = Secret::from_str(SECRET_STRING).unwrap();

        let log_string = "The secret is";

        let log_attempt = format!("{log_string} {secret:?}");

        assert!(
            !log_attempt.contains(SECRET_STRING),
            "Log attempt contains the secret string: `{log_attempt}`"
        );
        assert_eq!(
            log_attempt,
            format!("{log_string} REDACTED",),
            "Secret was not redacted in log attempt: `{log_attempt}`"
        );
    }

    #[test]
    fn test_inner() {
        let secret = Secret::from_str(SECRET_STRING).unwrap();

        assert_eq!(
            secret.secret(),
            SECRET_STRING,
            "Returned secret was not the provided secret: `{}`",
            secret.secret()
        );
    }
}
