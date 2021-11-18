pub struct EarlyNeedingScheme;

impl EarlyNeedingScheme {
    pub fn scheme<S: Into<String>>(self, scheme: S) -> EarlyNeedingHost {
        EarlyNeedingHost {
            scheme: scheme.into(),
        }
    }
}

pub struct EarlyNeedingHost {
    scheme: String,
}

impl EarlyNeedingHost {
    pub fn host<S: Into<String>>(self, host: S) -> Early {
        Early {
            host: host.into(),
            scheme: self.scheme,
            port: None,
        }
    }
}

pub struct Early {
    scheme: String,
    host: String,
    port: Option<u16>,
}

impl Early {
    pub fn new() -> EarlyNeedingScheme {
        EarlyNeedingScheme
    }

    pub fn port(self, port: u16) -> Self {
        Self {
            port: Some(port),
            ..self
        }
    }

    pub fn build(self) -> String {
        self.scheme + "://" + &self.host + &self.port.map(|p| format!(":{}", p)).unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::Early;

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

    #[test]
    fn can_add_port() {
        let url = Early::new()
            .scheme("http")
            .host("example.com")
            .port(8080)
            .build();
        assert_eq!("http://example.com:8080", url);
    }
}
