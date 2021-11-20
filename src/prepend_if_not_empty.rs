pub(crate) trait PrependIfNotEmpty {
    fn prepend_if_not_empty<S: Into<String>>(self, prepend: S) -> Self;
}

impl PrependIfNotEmpty for String {
    fn prepend_if_not_empty<S: Into<String>>(self, prepend: S) -> Self {
        if self.is_empty() {
            self
        } else {
            prepend.into() + &self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PrependIfNotEmpty;

    #[test]
    fn empty() {
        let s = "".to_owned();
        assert_eq!("", s.prepend_if_not_empty("xyzzy"));
    }

    #[test]
    fn non_empty() {
        let s = "bar".to_owned();
        assert_eq!("foobar", s.prepend_if_not_empty("foo"));
    }
}
