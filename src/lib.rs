struct Early {
    scheme: Option<String>,
    host: Option<String>,
}

impl Early {
    fn new() -> Self {
        Self {
            scheme: None,
            host: None,
        }
    }

    fn build(self) -> String {
        let scheme = self.scheme.map(|s| s + "://").unwrap_or("".into());
        scheme + &self.host.unwrap_or_default()
    }

    fn scheme(self, scheme: &str) -> Self {
        Early {
            scheme: Some(scheme.into()),
            ..self
        }
    }

    fn host(self, host: &str) -> Self {
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
    fn starts_empty() {
        let url = Early::new().build();
        assert_eq!("", url);
    }

    #[test]
    fn just_a_scheme() {
        let url = Early::new().scheme("http").build();
        assert_eq!("http://", url);
    }

    #[test]
    fn scheme_plus_host() {
        let url = Early::new().scheme("https").host("example.com").build();
        assert_eq!("https://example.com", url);
    }
}
