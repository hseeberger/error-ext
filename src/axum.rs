//! Error utilities for axum.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

/// Error that can be used as axum response, with an appropriate HTTP status code and – except for
/// `Internal` – with one or more error messages conveyed as a JSON string array.
#[derive(Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum Error {
    /// `400 Bad Request`, e.g. because of invalid path or query arguments.
    InvalidArgs(Vec<String>),

    /// `404 Not Found`.
    NotFound(String),

    /// `409 Conflict`, e.g. because of an already existing resource.
    Conflict(String),

    /// `422 Unprocessable Entity`, e.g. because of the JSON payload could not be parsed.
    InvalidEntity(Vec<String>),

    /// `500 Internal Server Error`.
    Internal,

    /// `503 Service Unavailable`.
    ServiceUnavailable,
}

impl Error {
    /// Create [Error::InvalidArgs] with the given error.
    pub fn invalid_args<T>(error: T) -> Self
    where
        T: ToString,
    {
        let errors = vec![error.to_string()];
        Error::InvalidArgs(errors)
    }

    /// Create [Error::InvalidArgs] with the given errors.
    pub fn invalid_args_all<I, T>(errors: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: ToString,
    {
        let errors = errors.into_iter().map(|e| e.to_string()).collect();
        Error::InvalidArgs(errors)
    }

    /// Create [Error::NotFound] with the given error.
    pub fn not_found<T>(error: T) -> Self
    where
        T: ToString,
    {
        Error::NotFound(error.to_string())
    }

    /// Create [Error::Conflict] with the given error.
    pub fn conflict<T>(error: T) -> Self
    where
        T: ToString,
    {
        Error::Conflict(error.to_string())
    }

    /// Create [Error::InvalidEntity] with the given error.
    pub fn invalid_entity<T>(error: T) -> Self
    where
        T: ToString,
    {
        let errors = vec![error.to_string()];
        Error::InvalidEntity(errors)
    }

    /// Create [Error::InvalidEntity] with the given errors.
    pub fn invalid_entity_all<I, T>(errors: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: ToString,
    {
        let errors = errors.into_iter().map(|e| e.to_string()).collect();
        Error::InvalidEntity(errors)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::InvalidArgs(errors) => {
                let errors = Json(
                    errors
                        .into_iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>(),
                );
                (StatusCode::BAD_REQUEST, errors).into_response()
            }

            Error::NotFound(error) => {
                let errors = Json(vec![error.to_string()]);
                (StatusCode::NOT_FOUND, errors).into_response()
            }

            Error::Conflict(error) => {
                let errors = Json(vec![error.to_string()]);
                (StatusCode::CONFLICT, errors).into_response()
            }

            Error::InvalidEntity(errors) => {
                let errors = Json(
                    errors
                        .into_iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>(),
                );
                (StatusCode::UNPROCESSABLE_ENTITY, errors).into_response()
            }

            Error::Internal => StatusCode::INTERNAL_SERVER_ERROR.into_response(),

            Error::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE.into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::iter;
    use thiserror::Error;

    #[derive(Debug, Error)]
    #[error("test")]
    struct TestError;

    #[test]
    fn test_invalid_args() {
        Error::invalid_args("test").into_response();
        Error::invalid_args(anyhow!("test")).into_response();
        let response = Error::invalid_args(TestError).into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_invalid_args_all() {
        Error::invalid_args_all(vec!["test"]).into_response();
        let response = Error::invalid_args_all(iter::once(TestError)).into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_not_found() {
        Error::not_found("test").into_response();
        Error::not_found(anyhow!("test")).into_response();
        let response = Error::not_found(TestError).into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_conflict() {
        Error::conflict("test").into_response();
        Error::conflict(anyhow!("test")).into_response();
        let response = Error::conflict(TestError).into_response();
        assert_eq!(response.status(), StatusCode::CONFLICT);
    }

    #[test]
    fn test_invalid_entity() {
        Error::invalid_entity("test").into_response();
        Error::invalid_entity(anyhow!("test")).into_response();
        let response = Error::invalid_entity(TestError).into_response();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn test_invalid_entity_all() {
        Error::invalid_entity_all(vec!["test"]).into_response();
        let response = Error::invalid_entity_all(iter::once(TestError)).into_response();
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[test]
    fn test_internal() {
        let response = Error::Internal.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
