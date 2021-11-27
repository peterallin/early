use itertools::Itertools;

mod extra_iters;
use extra_iters::ExtraIters;

#[derive(Default, Clone)]
pub struct Early {
    scheme: String,
    host: String,
    port: Option<u16>,
    paths: Vec<String>,
    query: Vec<(String, String)>,
}

impl Early {
    pub fn new<S1: Into<String>, S2: Into<String>>(scheme: S1, host: S2) -> Self {
        Self {
            scheme: scheme.into(),
            host: host.into(),
            ..Default::default()
        }
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
        self.query
            .iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .intersperse("&".into())
            .prepend_if_not_empty("?")
            .collect()
    }

    #[allow(unstable_name_collisions)] // I will be wanting to use the new function when it's available
    fn path_fragment(&self) -> String {
        self.paths
            .iter()
            .map(|p| urlencoding::encode(p).into_owned())
            .intersperse("/".into())
            .prepend_if_not_empty("/")
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Early;

    #[test]
    fn scheme_plus_host_str() {
        let url = Early::new("https", "example.com").build();
        assert_eq!("https://example.com", url);
    }

    #[test]
    fn scheme_plus_host_string() {
        let url = Early::new("https", "example.com".to_string()).build();
        assert_eq!("https://example.com", url);
    }

    #[test]
    fn can_add_port() {
        let url = Early::new("http", "example.com").port(8080).build();
        assert_eq!("http://example.com:8080", url);
    }

    #[test]
    fn can_add_single_query_key_value() {
        let url = Early::new("https", "example.com")
            .query("my_key", "my_value")
            .build();
        assert_eq!("https://example.com?my_key=my_value", url);
    }

    #[test]
    fn can_add_two_query_key_values() {
        let url = Early::new("https", "example.com")
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
        let url = Early::new("http", "example.com").path("foo").build();
        assert_eq!(url, "http://example.com/foo");
    }

    #[test]
    fn can_add_multiple_path_elements() {
        let url = Early::new("http", "example.com")
            .path("foo")
            .path("bar")
            .path("baz")
            .build();
        assert_eq!(url, "http://example.com/foo/bar/baz");
    }

    #[test]
    fn url_encodes_queries() {
        let url = Early::new("https", "example.com")
            .path("foo")
            .query("a b c", "d/e/f g")
            .build();
        assert_eq!(url, "https://example.com/foo?a%20b%20c=d%2Fe%2Ff%20g");
    }

    #[test]
    fn url_encodes_path() {
        let url = Early::new("http", "example.com").path("foo bar").build();
        assert_eq!(url, "http://example.com/foo%20bar");
    }

    #[test]
    fn enables_building_of_variant_urls() {
        let base = Early::new("https", "example.com")
            .path("api")
            .query("version", "42");
        let people_url = base.clone().path("people").build();
        let machines_url = base
            .clone()
            .path("machines")
            .query("type", "perpetual motion")
            .build();
        assert_eq!(people_url, "https://example.com/api/people?version=42");
        assert_eq!(
            machines_url,
            "https://example.com/api/machines?version=42&type=perpetual%20motion"
        );
    }
}
