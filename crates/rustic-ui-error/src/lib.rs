#![deny(missing_docs)]
#![doc = r"RusticUI shared error vocabulary
======================================

The RusticUI workspace spans multiple crates and host utilities. Keeping their
error handling aligned ensures automation can triage failures and contributors
receive consistent diagnostics. This crate exposes the shared [`RusticUiError`]
enum alongside helper traits so higher level crates can return typed
[`Result`](type.RusticUiResult.html) values instead of `anyhow::Error`.

Every variant is documented inline with runnable examples demonstrating how to
attach context and how the errors render when bubbled up to public APIs."]

use std::borrow::Cow;

/// Result alias routed through [`RusticUiError`].
pub type RusticUiResult<T> = Result<T, RusticUiError>;

/// Canonical error type used across the RusticUI workspace.
///
/// The variants intentionally map to the primary failure domains that the
/// automation crates interact with. Additional context can be layered on top
/// using [`ResultContextExt::context`], mirroring the ergonomics previously
/// offered by `anyhow::Context` but without erasing the concrete error type.
#[derive(Debug, thiserror::Error)]
pub enum RusticUiError {
    /// A plain error message without an underlying source.
    ///
    /// This is typically used when the error condition itself is terminal and
    /// does not have a lower level cause (for example validation failures).
    ///
    /// ```
    /// use rustic_ui_error::{RusticUiError, RusticUiResult};
    ///
    /// fn validate(flag: bool) -> RusticUiResult<()> {
    ///     if !flag {
    ///         return Err(RusticUiError::message("flag must be enabled"));
    ///     }
    ///     Ok(())
    /// }
    ///
    /// assert!(validate(true).is_ok());
    /// assert_eq!(
    ///     format!("{}", validate(false).unwrap_err()),
    ///     "flag must be enabled"
    /// );
    /// ```
    #[error("{message}")]
    Message {
        /// Human readable error message.
        message: Cow<'static, str>,
    },

    /// Wrapper for [`std::io::Error`].
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// [`serde_json::Error`] surfaced when JSON parsing fails.
    #[cfg(feature = "serde_json")]
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    /// [`walkdir::Error`] produced while traversing directory trees.
    #[cfg(feature = "walkdir")]
    #[error(transparent)]
    Walkdir(#[from] walkdir::Error),

    /// [`std::path::StripPrefixError`] emitted when relative path conversions fail.
    #[error(transparent)]
    StripPrefix(#[from] std::path::StripPrefixError),

    /// [`zip::result::ZipError`] raised while constructing archives.
    #[cfg(feature = "zip")]
    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    /// [`camino::FromPathBufError`] when converting OS paths into UTF-8 aware
    /// types fails.
    #[cfg(feature = "camino")]
    #[error(transparent)]
    Utf8Path(#[from] camino::FromPathBufError),

    /// [`cargo_metadata::Error`] emitted while inspecting the workspace graph.
    #[cfg(feature = "cargo_metadata")]
    #[error(transparent)]
    CargoMetadata(#[from] cargo_metadata::Error),

    /// [`ureq::Error`] encountered during HTTP requests.
    #[cfg(feature = "ureq")]
    #[error(transparent)]
    Http(#[from] ureq::Error),

    /// [`tokio::task::JoinError`] returned by asynchronous joins.
    #[cfg(feature = "tokio")]
    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),

    /// Contextualised error message that preserves the original source.
    ///
    /// This variant is primarily constructed through
    /// [`RusticUiError::with_context`] and [`ResultContextExt`].
    ///
    /// ```
    /// use rustic_ui_error::{ResultContextExt, RusticUiResult};
    /// use std::error::Error as _;
    ///
    /// fn load_file(path: &std::path::Path) -> RusticUiResult<String> {
    ///     std::fs::read_to_string(path)
    ///         .context("failed to load configuration file")
    /// }
    ///
    /// let err = load_file(std::path::Path::new("/definitely/missing.toml"))
    ///     .unwrap_err();
    /// assert!(format!("{err}").contains("failed to load configuration file"));
    /// assert!(err.source().unwrap().to_string().contains("No such file"));
    /// ```
    #[error("{message}")]
    Context {
        /// The contextual message that was layered on top of the source error.
        message: Cow<'static, str>,
        /// Underlying [`RusticUiError`] that triggered the failure.
        #[source]
        source: Box<RusticUiError>,
    },
}

impl RusticUiError {
    /// Construct a [`RusticUiError::Message`] variant.
    pub fn message(message: impl Into<Cow<'static, str>>) -> Self {
        RusticUiError::Message {
            message: message.into(),
        }
    }

    /// Layer additional context on top of an existing [`RusticUiError`].
    pub fn with_context(message: impl Into<Cow<'static, str>>, source: RusticUiError) -> Self {
        RusticUiError::Context {
            message: message.into(),
            source: Box::new(source),
        }
    }
}

/// Extension trait mirroring `anyhow::Context` but returning [`RusticUiResult`].
pub trait ResultContextExt<T> {
    /// Attach a context message to the error branch of the result.
    fn context<M>(self, message: M) -> RusticUiResult<T>
    where
        M: Into<Cow<'static, str>>;

    /// Lazily attach a context message to the error branch of the result.
    fn with_context<M, F>(self, message: F) -> RusticUiResult<T>
    where
        M: Into<Cow<'static, str>>,
        F: FnOnce() -> M;
}

impl<T, E> ResultContextExt<T> for Result<T, E>
where
    E: Into<RusticUiError>,
{
    fn context<M>(self, message: M) -> RusticUiResult<T>
    where
        M: Into<Cow<'static, str>>,
    {
        self.map_err(|err| RusticUiError::with_context(message, err.into()))
    }

    fn with_context<M, F>(self, message: F) -> RusticUiResult<T>
    where
        M: Into<Cow<'static, str>>,
        F: FnOnce() -> M,
    {
        self.map_err(|err| RusticUiError::with_context(message(), err.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn context_preserves_source_message() {
        let result: RusticUiResult<()> = Err(RusticUiError::from(std::io::Error::from(
            std::io::ErrorKind::NotFound,
        )))
        .context("top-level context");
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "top-level context");
        let source = err.source().unwrap();
        assert!(source.to_string().contains("found"));
    }
}
