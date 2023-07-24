use crate::utils::errors::{ApiError, JsonIncoming};
use axum::{http::StatusCode, Extension};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, Pool};

pub async fn post_signup(
    Extension(database): Extension<Pool<MySql>>,
    JsonIncoming(payload): JsonIncoming<AccountOptions>,
) -> Result<(), (StatusCode, axum::Json<ApiError>)> {
    Ok(())
}

#[derive(Deserialize)]
pub struct AccountOptions {
    name: String,
}

#[derive(Serialize)]
pub struct AccountCreationSuccess {
    token: String,
    id: String,
}
