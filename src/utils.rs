use axum::{response::{IntoResponse, Response}, Json, http::StatusCode}; 
/// A collection of errors that can occur during the execution of the app. 
/// Note: these are errors that do not stop execution of the app. 
/// The implementations of From<T> for AppError allow using "?" to conveniently map errors of one kind to another. 
pub enum AppError {
    RequestError(reqwest::Error),
    WeatherAPIError(String),
    RedisError(redis::RedisError), 
    ParsingError(serde_json::Error), 
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        Self::RequestError(error)
    }
}

impl From<&str> for AppError {
    fn from(value: &str) -> Self {
        Self::WeatherAPIError(value.to_string())
    }
}

impl From<redis::RedisError> for AppError  {
    fn from(error: redis::RedisError) -> Self {
        Self::RedisError(error)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        Self::ParsingError(error)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::ParsingError(error) => (StatusCode::BAD_REQUEST, error.to_string()),
            AppError::RedisError(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::RequestError(error) => (StatusCode::BAD_GATEWAY, error.to_string()),
            AppError::WeatherAPIError(error) => (StatusCode::BAD_REQUEST, error),
        }; 

        let error_response = serde_json::json!({
            "status": "error",
            "message": message
        }); 

        (status, Json(error_response)).into_response()

    }
}