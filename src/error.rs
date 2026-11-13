use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;
use crate::plugin::plugin::FeatureKind;

#[derive(Debug, Error)]
pub enum AppError {
    /// Placeholder for logic that has not been ported from Endurance yet.
    #[error("{0} is not implemented yet")]
    NotImplemented(&'static str),
    
    #[error("Plugin {0} not found")]
    PluginNotFound(String),
    
    #[error("plugin error: {cause}")]
    PluginError {
        #[from]
        cause: PluginError,
    },
}

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("Plugin failed for unknown error: {0}")]
    Unknown(String),
    #[error("Unsupported feature: {0:?} - supported features {1:?}")]
    UnsupportedFeature(FeatureKind, Vec<FeatureKind>),
}

#[derive(Serialize)]
struct ErrorBody {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::NotImplemented(_) => StatusCode::NOT_IMPLEMENTED,
            AppError::PluginNotFound(_) => StatusCode::NOT_FOUND,
            AppError::PluginError { cause: PluginError::UnsupportedFeature(_, _) } => StatusCode::NOT_FOUND,
            AppError::PluginError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(ErrorBody { message: self.to_string() })).into_response()
    }
}
