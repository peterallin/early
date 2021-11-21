use std::iter::Peekable;

pub(crate) trait ExtraIters {
    fn prepend_if_not_empty<S>(self, prepend: S) -> PrependIfNotEmpty<Self>
    where
        S: Into<String>,
        Self: Sized + Iterator,
    {
        PrependIfNotEmpty {
            iter: self.peekable(),
            prepend: Some(prepend.into()),
        }
    }
}

impl<I> ExtraIters for I where I: Iterator {}

pub struct PrependIfNotEmpty<Iter: Iterator> {
    iter: Peekable<Iter>,
    prepend: Option<String>,
}

impl<Iter> Iterator for PrependIfNotEmpty<Iter>
where
    Iter: Iterator<Item = String>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.prepend.is_some() && self.iter.peek().is_some() {
            self.prepend.take()
        } else {
            self.iter.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ExtraIters;

    #[test]
    fn prepends_to_non_empty() {
        let v = vec!["bar".to_string(), "baz".to_string()]
            .into_iter()
            .prepend_if_not_empty("foo");
        let v: Vec<_> = v.collect();
        assert_eq!(v, vec!["foo", "bar", "baz"]);
    }

    #[test]
    fn does_not_prepend_to_empty() {
        let v = vec![].into_iter().prepend_if_not_empty("foo");
        let v: Vec<String> = v.collect();
        let empty: Vec<String> = vec![];
        assert_eq!(v, empty);
    }
}
