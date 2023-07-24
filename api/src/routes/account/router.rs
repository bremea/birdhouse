use axum::{routing::post, Router};

use super::signup::post_signup;

pub fn router() -> Router {
    return Router::new().route("/signup", post(post_signup));
}
