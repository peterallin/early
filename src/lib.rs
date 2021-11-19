use itertools::Itertools;

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
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct Early {
    scheme: String,
    host: String,
    port: Option<u16>,
    paths: Vec<String>,
    query: Vec<(String, String)>,
}

impl Early {
    #[allow(clippy::new_ret_no_self)] // Clippy is right, but I want it to look like there's only one type involved... might be a bad idea
    pub fn new() -> EarlyNeedingScheme {
        EarlyNeedingScheme
    }

    pub fn port(self, port: u16) -> Self {
        Self {
            port: Some(port),
            ..self
        }
    }

    pub fn path<S: Into<String>>(mut self, path: S) -> Self {
        self.paths.push(path.into());
        self
    }

    pub fn query<S: Into<String>>(mut self, key: S, value: S) -> Self {
        self.query.push((key.into(), value.into()));
        self
    }

    pub fn build(self) -> String {
        let port = self.port_fragment();
        let query = self.query_fragment();
        let path = self.path_fragment();
        self.scheme + "://" + &self.host + &port + &path + &query
    }

    fn port_fragment(&self) -> String {
        self.port.map(|p| format!(":{}", p)).unwrap_or_default()
    }

    #[allow(unstable_name_collisions)] // I will be wanting to use the new function when it's available
    fn query_fragment(&self) -> String {
        let query: String = self
            .query
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .intersperse("&".into())
            .collect();
        if query.is_empty() {
            query
        } else {
            "?".to_owned() + &query
        }
    }

    #[allow(unstable_name_collisions)] // I will be wanting to use the new function when it's available
    fn path_fragment(&self) -> String {
        let path: String = self.paths.iter().cloned().intersperse("/".into()).collect();
        if path.is_empty() {
            path
        } else {
            "/".to_owned() + &path
        }
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

    #[test]
    fn can_add_single_query_key_value() {
        let url = Early::new()
            .scheme("https")
            .host("example.com")
            .query("my_key", "my_value")
            .build();
        assert_eq!("https://example.com?my_key=my_value", url);
    }

    #[test]
    fn can_add_two_query_key_values() {
        let url = Early::new()
            .scheme("https")
            .host("example.com")
            .query("my_key1", "my_value1")
            .query("my_key2", "my_value2")
            .build();
        assert_eq!(
            "https://example.com?my_key1=my_value1&my_key2=my_value2",
            url
        );
    }

    #[test]
    fn can_add_single_path_element() {
        let url = Early::new()
            .scheme("http")
            .host("example.com")
            .path("foo")
            .build();
        assert_eq!(url, "http://example.com/foo");
    }
    #[test]
    fn can_add_multiple_path_elements() {
        let url = Early::new()
            .scheme("http")
            .host("example.com")
            .path("foo")
            .path("bar")
            .path("baz")
            .build();
        assert_eq!(url, "http://example.com/foo/bar/baz");
    }
}
