
// Converting responses

use crate::Response;

pub trait IntoResponse: Send + Sized {
    fn into_response(self) -> Response;
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> Response {
        Response::new(self.into())
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Response::new(self.into())
    }
}