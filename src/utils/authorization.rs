use crate::{errors::Error, models::user::User};
use axum::http::HeaderMap;
use reqwest;
use serde::{Deserialize, Serialize};

pub async fn authorize(headers: HeaderMap) -> Result<User, Error> {
    let token = match get_token_from_headers(headers).await {
        Ok(token) => token,
        Err(error) => return Err(error),
    };

    let user = match verify_auth_token(token).await {
        Ok(user) => user,
        Err(error) => return Err(error),
    };

    return Ok(user);
}

pub async fn get_token_from_headers(headers: HeaderMap) -> Result<String, Error> {
    let auth_header = headers.get(AUTHORIZATION_HEADER_KEY);
    match auth_header {
        None => return Err(Error::unauthorized()),
        Some(header) => {
            let token_string = header.to_str().unwrap().to_string();

            if !token_string.to_string().starts_with("Bearer") {
                return Err(Error::unauthorized());
            }

            return Ok(token_string[7..].to_string());
        }
    };
}

pub async fn verify_auth_token(access_token: String) -> Result<User, Error> {
    let client = reqwest::Client::new();

    let params = [(GOOGLE_ACCESS_TOKEN_KEY, access_token)];

    match client
        .get("https://oauth2.googleapis.com/tokeninfo")
        .query(&params)
        .send()
        .await
    {
        Ok(response) => {
            match response.status() {
                reqwest::StatusCode::OK => {
                    match response.json::<GoogleOAuthClaims>().await {
                        Ok(parsed) => {
                            return Ok(User {
                                email: parsed.email,
                                name: parsed.name,
                            });
                        }
                        Err(_) => return Err(Error::unauthorized()),
                    };
                }
                reqwest::StatusCode::BAD_REQUEST => {
                    log::error!("Couldn't verify Google authentication token");
                    log::error!("Response: {:?}", response);
                    return Err(Error::unauthorized());
                }
                _ => {
                    log::error!("Unexpected occurred when verifying Google authentication token");
                    log::error!("Response: {:?}", response);
                    return Err(Error::unauthorized());
                }
            };
        }
        Err(_) => return Err(Error::unauthorized()),
    };
}

static AUTHORIZATION_HEADER_KEY: &str = "Authorization";
static GOOGLE_ACCESS_TOKEN_KEY: &str = "id_token";

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleOAuthClaims {
    pub iss: String,
    pub azp: String,
    pub aud: String,
    pub sub: String,
    pub email: String,
    pub email_verified: String,
    pub nbf: String,
    pub name: String,
    pub picture: String,
    pub given_name: String,
    pub family_name: String,
    pub locale: String,
    pub iat: String,
    pub exp: String,
    pub alg: String,
    pub kid: String,
    pub typ: String,
}
