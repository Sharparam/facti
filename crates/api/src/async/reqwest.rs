use std::{borrow::Cow, fs, io, path::Path};

use reqwest::multipart::{Form, Part};
use thiserror::Error;

use crate::reqwest::FormLike;

/// Sets up wrapper methods to interact with [`reqwest::multipart::Form`]
/// when contained in a [`crate::reqwest::FormContainer`].
#[doc(hidden)]
impl FormLike for reqwest::multipart::Form {
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

// Workaround for weird decision in reqwest API design

#[doc(hidden)]
#[derive(Error, Debug)]
pub(crate) enum AsyncFormFileError {
    #[error("IO error during form file operation")]
    Io(io::Error),

    #[error("reqwest error during form file operation")]
    Reqwest(reqwest::Error),
}

#[doc(hidden)]
pub(crate) trait AsyncFormFile {
    fn file<T, U>(self, name: T, path: U) -> core::result::Result<Form, AsyncFormFileError>
    where
        T: Into<Cow<'static, str>>,
        U: AsRef<Path>;
}

impl AsyncFormFile for Form {
    fn file<T, U>(self, name: T, path: U) -> core::result::Result<Form, AsyncFormFileError>
    where
        T: Into<Cow<'static, str>>,
        U: AsRef<Path>,
    {
        let path = path.as_ref();
        let file_name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        let mime = mime_guess::from_ext(ext).first_or_octet_stream();
        let mime_str = mime.essence_str();
        let file = fs::read(path).map_err(AsyncFormFileError::Io)?;
        let file_part = Part::bytes(file)
            .file_name(file_name)
            .mime_str(mime_str)
            .map_err(AsyncFormFileError::Reqwest)?;

        Ok(self.part(name, file_part))
    }
}
