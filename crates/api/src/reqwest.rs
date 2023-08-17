use std::borrow::Cow;

pub(crate) trait FormLike {
    fn new() -> Self;

    fn text<T, U>(self, name: T, value: U) -> Self
    where
        T: Into<Cow<'static, str>>,
        U: Into<Cow<'static, str>>;
}

pub(crate) struct FormContainer<T: FormLike>(pub(crate) T);

impl<T: FormLike> FormContainer<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}
