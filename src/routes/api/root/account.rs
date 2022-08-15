use rocket::serde::json::Json;
use rocket::*;

use misato_database::{database::*, models::*};

use misato::models::account_model;

use crate::errors::account_errors;
use crate::fairings::api_authentication::ApiUserToken;
use crate::fairings::authentication::UserToken;

const TOKEN_DURATION: u64 = 24 * 60 * 60;

#[post("/api/signup")]
pub async fn signup(
    user: UserToken,
    db: &State<Database>,
) -> Result<Json<account_model::AccountTokenInfos>, account_errors::Error> {
    let user = user.user;

    let result = db
        .apiusermanager
        .get_apiuser(None, Some(&user.uuid.to_string()))
        .await;
    if result.is_ok() && result.as_ref().unwrap().is_some() {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(
                400,
                Some(format!("[{}]: API Account already exists.", user.uuid)),
            ),
        });
    }
    if result.is_err() {
        println!("{:?}", result.unwrap_err());
        return Err(account_errors::Error {
            content: account_model::AccountError::build(500, Some("Database error.".to_string())),
        });
    }

    let mut apiuser = apiuser_model::ApiUser::create(user.uuid.clone());

    match db.apiusermanager.create_apiuser(&apiuser).await {
        Ok(_) => {
            let token = apiuser.new_token(TOKEN_DURATION);
            match db.apiusermanager.set_token(&user.uuid, &token).await {
                Ok(_) => {
                    return Ok(Json(account_model::AccountTokenInfos {
                        token: token.token,
                        timestamp: token.timestamp,
                        expiration_timestamp: token.expiration_timestamp,
                    }))
                }
                Err(_error) => {
                    println!("{:?}", _error);
                    return Err(account_errors::Error {
                        content: account_model::AccountError::build(
                            500,
                            Some("Database error.".to_string()),
                        ),
                    });
                }
            }
        }
        Err(_error) => {
            println!("{:?}", _error);
            return Err(account_errors::Error {
                content: account_model::AccountError::build(
                    500,
                    Some("Database error.".to_string()),
                ),
            });
        }
    }
}

#[post("/api/refresh-token")]
pub async fn refresh_token(
    user: UserToken,
    db: &State<Database>,
) -> Result<Json<account_model::AccountTokenInfos>, account_errors::Error> {
    let user = user.user;

    let result = db
        .apiusermanager
        .get_apiuser(None, Some(&user.uuid.to_string()))
        .await;
    if result.is_ok() && result.as_ref().unwrap().is_none() {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(
                400,
                Some(format!("[{}]: API Account doesn't exist.", user.uuid)),
            ),
        });
    }
    if result.is_err() {
        println!("{:?}", result.unwrap_err());
        return Err(account_errors::Error {
            content: account_model::AccountError::build(500, Some("Database error.".to_string())),
        });
    }
    let token = result.unwrap().unwrap().new_token(TOKEN_DURATION);
    match db.apiusermanager.set_token(&user.uuid, &token).await {
        Ok(_) => {
            return Ok(Json(account_model::AccountTokenInfos {
                token: token.token,
                timestamp: token.timestamp,
                expiration_timestamp: token.expiration_timestamp,
            }))
        }
        Err(_error) => {
            println!("{:?}", _error);
            return Err(account_errors::Error {
                content: account_model::AccountError::build(
                    500,
                    Some("Database error.".to_string()),
                ),
            });
        }
    }
}

#[post("/api/check-token")]
pub async fn check_token(
    api: ApiUserToken,
) -> Result<Json<account_model::AccountTokenInfos>, account_errors::Error> {
    let token = api.apiuser.token.unwrap();
    return Ok(Json(account_model::AccountTokenInfos {
        token: token.token,
        timestamp: token.timestamp,
        expiration_timestamp: token.expiration_timestamp,
    }));
}

#[post("/api/delete")]
pub async fn delete(
    api: ApiUserToken,
    db: &State<Database>,
) -> Result<Json<String>, account_errors::Error> {
    let token = api.apiuser.token.unwrap().token;
    match db.apiusermanager.delete_apiuser_from_token(&token).await {
        Ok(_) => {
            return Ok(Json("Account deleted.".to_string()));
        }
        Err(error) => {
            println!("{:?}", error);
            return Err(account_errors::Error {
                content: account_model::AccountError::build(
                    500,
                    Some("Database error.".to_string()),
                ),
            });
        }
    }
}

#[post("/api/clear-tokens")]
pub async fn clear_tokens(
    api: ApiUserToken,
    db: &State<Database>,
) -> Result<Json<String>, account_errors::Error> {
    let token = api.apiuser.token.unwrap().token;
    match db.apiusermanager.clear_tokens_from_token(&token).await {
        Ok(_) => {
            return Ok(Json("Token removed.".to_string()));
        }
        Err(error) => {
            println!("{:?}", error);
            return Err(account_errors::Error {
                content: account_model::AccountError::build(
                    500,
                    Some("Database error.".to_string()),
                ),
            });
        }
    }
}
