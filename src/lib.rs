pub struct Early {
    scheme: Option<String>,
    host: Option<String>,
}

pub struct EarlyNeedingScheme;

impl EarlyNeedingScheme {
    pub fn scheme<S: Into<String>>(self, scheme: S) -> Early {
        Early {
            scheme: Some(scheme.into()),
            host: None,
        }
    }
}

impl Early {
    pub fn new() -> EarlyNeedingScheme {
        EarlyNeedingScheme
    }

    pub fn build(self) -> String {
        let scheme = self.scheme.map(|s| s + "://").unwrap_or("".into());
        scheme + &self.host.unwrap_or_default()
    }

    pub fn host<S: Into<String>>(self, host: S) -> Self {
        Early {
            host: Some(host.into()),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Early;

    #[test]
    fn just_a_scheme_str() {
        let url = Early::new().scheme("http").build();
        assert_eq!("http://", url);
    }

    #[test]
    fn just_a_scheme_string() {
        let url = Early::new().scheme("http".to_string()).build();
        assert_eq!("http://", url);
    }

    #[test]
    fn scheme_plus_host_str() {
        let url = Early::new()
            .scheme("https")
            .host("example.com")
            .build();
        assert_eq!("https://example.com", url);
    }

    #[test]
    fn scheme_plus_host_string() {
        let url = Early::new()
            .scheme("https")
            .host("example.com".to_string())
            .build();
        assert_eq!("https://example.com", url);
    }
}
