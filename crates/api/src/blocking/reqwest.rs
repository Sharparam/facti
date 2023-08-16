use std::borrow::Cow;

use crate::reqwest::FormLike;

impl FormLike for reqwest::blocking::multipart::Form {
    fn new() -> Self {
        Self::new()
    }

    fn text<T, U>(self, name: T, value: U) -> Self
    where
        T: Into<Cow<'static, str>>,
        U: Into<Cow<'static, str>>,
    {
        self.text(name, value)
    }
}
