use actix_web::{error::InternalError, web, HttpResponse, cookie::Cookie};
use hmac::{Hmac, Mac};
use reqwest::header::LOCATION;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

use crate::{
  authentication::{validate_credentials, Credentials},
  routes::error_chain_fmt,
  startup::HmacSecret,
};

#[derive(serde::Deserialize)]
pub struct FormData {
  username: String,
  password: Secret<String>,
}

pub async fn login(
  form: web::Form<FormData>,
  pool: web::Data<PgPool>,
  secret: web::Data<HmacSecret>,
) -> Result<HttpResponse, InternalError<LoginError>> {
  let credentials = Credentials {
    username: form.0.username,
    password: form.0.password,
  };
  tracing::Span::current().record("username", &tracing::field::display(&credentials.username));
  match validate_credentials(credentials, &pool).await {
    Ok(user_id) => {
      tracing::Span::current().record("user_id", &tracing::field::display(&user_id));
      Ok(
        HttpResponse::SeeOther()
          .insert_header((LOCATION, "/"))
          .finish(),
      )
    }
    Err(e) => {
      let e = match e {
        crate::authentication::AuthError::InvalidCredentials(_) => LoginError::AuthError(e.into()),
        crate::authentication::AuthError::UnexpectedError(_) => {
          LoginError::UnexpectedError(e.into())
        }
      };
      let response = HttpResponse::SeeOther()
        .insert_header((LOCATION, "/login"))
        .cookie(Cookie::new("_flash", e.to_string()))
        .finish();
      Err(InternalError::from_response(e, response))
    }
  }
}

#[derive(thiserror::Error)]
pub enum LoginError {
  #[error("Authentication failed")]
  AuthError(#[source] anyhow::Error),
  #[error("Something went wrong")]
  UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for LoginError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    error_chain_fmt(self, f)
  }
}