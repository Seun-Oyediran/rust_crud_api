use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub success: bool,
}

impl ErrorResponse {
    pub fn new(message: String) -> Self {
        ErrorResponse {
            message,
            success: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    data: T,
    success: bool,
    message: String,
}

impl<T> SuccessResponse<T> {
    pub fn new(data: T, message: String) -> Self {
        SuccessResponse {
            data,
            success: true,
            message,
        }
    }
}
