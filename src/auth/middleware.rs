use super::{
    error::{AuthError, AuthResult},
    model::{FilteredUser, TokenClaims},
};
use crate::{
    database::handlers::*,
    error::{DataApiReturn, ErrorResponse, IntoDataApiReturn},
    AppState,
};
use axum::{
    body::Body,
    extract::State,
    http::{header, Request},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use reqwest::StatusCode;
use std::sync::Arc;
use tracing::{info, warn};

pub fn get_user_id(
    jwt_secret: &[u8],
    cookie_jar: CookieJar,
    req: &Request<Body>,
) -> AuthResult<uuid::Uuid> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        })
        .ok_or(AuthError::NotLoggedIn)?;
    warn!("Token got from cookie jar: {:?}", token);

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(jwt_secret),
        &Validation::default(),
    )?
    .claims;

    let user_id = uuid::Uuid::parse_str(&claims.sub)?;

    Ok(user_id)
}

pub async fn auth(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, DataApiReturn> {
    let user_id = get_user_id(data.env.jwt_secret.as_ref(), cookie_jar, &req)
        .map_err(|err| err.into_data_api_return())?;

    let user = get_user_by_id(&data.db, user_id)
        .await
        .map_err(|e| e.into_data_api_return())?
        .ok_or_else(|| AuthError::NoLongerExists.into_data_api_return())?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub async fn admin_auth(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, DataApiReturn> {
    let user_id = get_user_id(data.env.jwt_secret.as_ref(), cookie_jar, &req)
        .map_err(|err| err.into_data_api_return())?;

    let user = get_user_by_id(&data.db, user_id)
        .await
        .map_err(|e| e.into_data_api_return())?
        .ok_or_else(|| AuthError::NoLongerExists.into_data_api_return())?;
    if !user.admin {
        return Err((
            StatusCode::UNAUTHORIZED,
            axum::Json(ErrorResponse::fail(&format!("Not an admin"))),
        ));
    }

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

#[derive(Clone)]
pub struct SoftAuthExtension {
    pub user: Option<FilteredUser>,
}

pub async fn soft_auth(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, DataApiReturn> {
    info!("In soft auth");
    let user = {
        if let Some(user_id) = get_user_id(data.env.jwt_secret.as_ref(), cookie_jar, &req)
            .map_err(|err| anyhow::anyhow!(err))
            .ok()
        {
            match get_user_by_id(&data.db, user_id)
                .await
                .map_err(|e| e.into_data_api_return())?
            {
                Some(user) => {
                    info!("soft auth got user");
                    Some(FilteredUser::from(&user))
                }
                None => {
                    info!("soft auth got no user because user no longer exists");
                    None
                }
            }
        } else {
            info!("soft auth got no user");
            None
        }
    };
    req.extensions_mut().insert(SoftAuthExtension { user });

    Ok(next.run(req).await)
}
