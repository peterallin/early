struct Early {
    scheme: Option<String>,
}

impl Early {
    fn new() -> Self {
        Self { scheme: None }
    }

    fn build(self) -> String {
        self.scheme.map(|s| s + "://").unwrap_or("".into())
    }

    fn scheme(self, scheme: &str) -> Self {
        Early {
            scheme: Some(scheme.into()),
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
}
