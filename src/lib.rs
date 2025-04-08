#![cfg_attr(docsrs, feature(doc_cfg))]

//! Error utilities.

#[cfg(feature = "axum")]
#[cfg_attr(docsrs, doc(cfg(feature = "axum")))]
pub mod axum;

use std::error::Error as StdError;

/// Convenience for `async` and `anyhow` friendly dynamic error
/// `Box<dyn std::error::Error + Send + Sync + 'static>`.
pub type BoxError = Box<dyn StdError + Send + Sync + 'static>;

/// Extension methods for std errors.
pub trait StdErrorExt
where
    Self: StdError,
{
    /// Format this error as a chain of colon separated strings built from this error and all
    /// recursive sources.
    ///
    /// Can be used to log errors like this:
    ///
    /// `error!(error = error.as_chain(), "cannot do this or that");`
    fn as_chain(&self) -> String {
        let mut sources = vec![];
        sources.push(self.to_string());

        let mut source = self.source();
        while let Some(s) = source {
            sources.push(s.to_string());
            source = s.source();
        }

        sources.join(": ")
    }

    /// Converts this error – given it implements `Sized` + `Send` +`Sync` and `'static` – into a
    /// [BoxError].
    fn into_boxed(self) -> BoxError
    where
        Self: Sized + Send + Sync + 'static,
    {
        self.into()
    }
}

impl<T> StdErrorExt for T where T: StdError {}

#[cfg(test)]
mod tests {
    use super::*;
    use thiserror::Error;

    #[derive(Debug, Error)]
    #[error("outer")]
    struct Outer(#[source] Inner);

    #[derive(Debug, Error)]
    #[error("inner")]
    struct Inner;

    #[test]
    fn test_as_chain() {
        let error = Outer(Inner);
        assert_eq!(error.as_chain(), "outer: inner");
    }

    #[test]
    fn test_into_boxed() {
        let error = Outer(Inner);
        let error = error.into_boxed();
        assert_eq!(error.to_string(), "outer");
    }
}
