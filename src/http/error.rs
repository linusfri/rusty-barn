use axum::extract::multipart::MultipartError;
use derive_more::Display;

pub type AppResult<T, E = WebError>  = axum::response::Result<T, E>;

#[derive(Debug, Display)]
#[display(fmt = "Status code: {}\nMessage: {}", status, message)]
pub struct WebError {
    pub(crate) status: axum::http::StatusCode,
    pub(crate) message: String
}

impl WebError {
    pub fn new(status: axum::http::StatusCode, message: String) -> Self {
        Self {
            status,
            message
        }
    }
}

impl From<MultipartError> for WebError {
    fn from(err: MultipartError) -> Self {
        WebError::new(axum::http::StatusCode::BAD_REQUEST, err.to_string())
    }
}

impl From<std::io::Error> for WebError {
    fn from(err: std::io::Error) -> Self {
        WebError::new(axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}

impl From<image::ImageError> for WebError {
    fn from(err: image::ImageError) -> Self {
        WebError::new(axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}

impl From<tera::Error> for WebError {
    fn from(err: tera::Error) -> Self {
        WebError::new(axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}

impl std::error::Error for WebError {}

impl axum::response::IntoResponse for WebError {
    fn into_response(self) -> axum::response::Response {
        (self.status, self.message).into_response()
    }
}