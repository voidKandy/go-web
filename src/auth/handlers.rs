use super::{
    error::AuthResult,
    model::{FilteredUser, LoginUserSchema, RegisterUserSchema, TokenClaims},
};
use crate::{
    auth::error::AuthError,
    database::{handlers::*, model::*},
    error::{DataApiReturn, ErrorResponse, IntoDataApiReturn},
    AppState,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Form, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand_core::OsRng;
use serde_json::json;
use std::sync::Arc;
use tracing::{error, info};

pub fn hash_password(password: &str) -> AuthResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|err| AuthError::from(err))?
        .to_string();
    Ok(hashed_password)
}

pub async fn register_user_handler(
    State(data): State<Arc<AppState>>,
    Form(body): Form<RegisterUserSchema>,
) -> Result<impl IntoResponse, DataApiReturn> {
    info!("Register request body: {:?}", body);
    let user_opt = get_user_by_email(&data.db, &body.email.trim())
        .await
        .map_err(|e| e.into_data_api_return())?;

    if user_opt.is_some() {
        let error_response = ErrorResponse::fail("User with that email already exists");
        return Err((StatusCode::CONFLICT, Json(error_response)));
    }

    let user = register_new_user(
        &data.db,
        &body.name,
        &body.email,
        body.subscribed,
        &hash_password(&body.password).map_err(|err| err.into_data_api_return())?,
        false,
    )
    .await
    .map_err(|e| e.into_data_api_return())?;

    let user_response = serde_json::json!({"status": "success","data": serde_json::json!({
        "user": FilteredUser::from(&user)
    })});

    Ok(Json(user_response))
}

pub async fn login_user_handler(
    State(data): State<Arc<AppState>>,
    Form(body): Form<LoginUserSchema>,
) -> Result<impl IntoResponse, DataApiReturn> {
    info!("Login request Body {:?}", body);

    let user = get_user_by_email(&data.db, &body.email.trim())
        .await
        .map_err(|e| e.into_data_api_return())?
        .ok_or_else(|| AuthError::NoLongerExists.into_data_api_return())?;

    let is_valid = match PasswordHash::new(&user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = ErrorResponse::fail("Invalid email or password");
        error!("Error logging in: {:?}", error_response);
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn logout_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    Ok(response)
}

pub async fn get_me_handler(
    Extension(user): Extension<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status":  "success",
        "data": serde_json::json!({
            "user": FilteredUser::from(&user)
        })
    });

    Ok(Json(json_response))
}

// mod tests {
//     use super::*;
//     #[test]
//     fn gen_pass() {
//         let salt = SaltString::generate(&mut OsRng);
//         let hashed_password = Argon2::default()
//             .hash_password("laptop-steam-boat-horse".as_bytes(), &salt)
//             .map_err(|err| AuthError::from(err).into_data_api_return())
//             .unwrap()
//             .to_string();
//         println!("HASH: [{}]", hashed_password);
//
//         let is_valid = match PasswordHash::new(&hashed_password) {
//             Ok(parsed_hash) => {
//                 println!("PARSED: [{}]", parsed_hash);
//                 Argon2::default()
//                     .verify_password("laptop-steam-boat-horse".as_bytes(), &parsed_hash)
//                     .map_or(false, |_| true)
//             }
//             Err(_) => false,
//         };
//         assert!(is_valid);
//         assert!(false);
//     }
// }
