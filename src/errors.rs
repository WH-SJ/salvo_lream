use salvo::prelude::*;
use salvo::{
    async_trait,
    http::{StatusCode, StatusError},
    oapi, Response, Writer,
};
use sea_orm::DbErr;
use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

/// 统一错误处理
#[derive(Error, Debug)]
pub enum AppError {
    /// 数据库错误
    #[error("Database Error: {0}")]
    DbError(#[from] DbErr),

    /// 验证错误
    #[error("Validation Failed: {0}")]
    ValidationError(String),

    /// 认证错误
    #[error("Authentication Required")]
    Unauthorized,

    /// 权限错误
    #[error("Forbidden: {0}")]
    Forbidden(String),

    /// 资源不存在
    #[error("Resource Not Found")]
    NotFound(String),

    /// 内部服务错误
    #[error("Internal Server Error")]
    InternalServerError(String),

    /// 业务错误
    #[error("Business Error: {0}")]
    BusinessError(String),
    
    ///参数不足
    #[error("Insufficient Parameters: {0}")]
    InsufficientParameters(String),
}
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    code: u16,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl AppError {
    /// 获取对应的 HTTP 状态码
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) | Self::InsufficientParameters(_) => StatusCode::NOT_FOUND,
            Self::BusinessError(_) => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn to_response(&self) -> ErrorResponse {
        self.into()
    }
}

#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let status_code = self.status_code();
        let error_response = self.to_response();

        res.status_code(status_code);
        res.render(Json(error_response));
    }
}

impl From<StatusError> for AppError {
    fn from(err: StatusError) -> Self {
        match err.code {
            StatusCode::UNAUTHORIZED => Self::Unauthorized,
            StatusCode::FORBIDDEN => Self::Forbidden(err.to_string()),
            StatusCode::NOT_FOUND => Self::NotFound("".to_string()),
            _ => Self::InternalServerError("".to_string()),
        }
    }
}

impl From<&AppError> for ErrorResponse {
    fn from(a: &AppError) -> Self {
        ErrorResponse {
            code: a.status_code().as_u16(),
            message: a.to_string(),
            details: match a {
                AppError::ValidationError(details) => {
                    Some(serde_json::json!({ "errors": details }))
                }
                _ => None,
            },
        }
    }
}

impl EndpointOutRegister for AppError {
    fn register(components: &mut oapi::Components, operation: &mut oapi::Operation) {
        operation.responses.insert(
            StatusCode::INTERNAL_SERVER_ERROR.as_str(),
            oapi::Response::new("Internal server error")
                .add_content("application/json", StatusError::to_schema(components)),
        );
        operation.responses.insert(
            StatusCode::NOT_FOUND.as_str(),
            oapi::Response::new("Not found")
                .add_content("application/json", StatusError::to_schema(components)),
        );
        operation.responses.insert(
            StatusCode::UNAUTHORIZED.as_str(),
            oapi::Response::new("Unauthorized")
                .add_content("application/json", StatusError::to_schema(components)),
        );
    }
}
