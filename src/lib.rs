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
        }
    }
}

pub struct Early {
    scheme: String,
    host: String,
}

impl Early {
    pub fn new() -> EarlyNeedingScheme {
        EarlyNeedingScheme
    }

    pub fn build(self) -> String {
        self.scheme + "://" + &self.host
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
}
