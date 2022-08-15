use rocket::serde::json::Json;
use rocket::*;

use misato::models::account_model;

use misato_database::database::*;

use misato_database::models::apiuser_model::ApiUserRoleType;

use crate::errors::account_errors;

use crate::fairings::api_authentication::ApiUserToken;

#[post("/user/check-token", data = "<input>")]
pub async fn check_token(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountToken>,
) -> Result<Json<account_model::AccountTokenInfos>, account_errors::Error> {
    let api = api.apiuser;

    let result = db.usermanager.get_user(None, Some(&api.uuid)).await;
    if result.is_ok() && result.as_ref().unwrap().is_none() {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(
                400,
                Some(format!("[{}]: User account doesn't exist.", api.uuid)),
            ),
        });
    }
    match api.access.role {
        ApiUserRoleType::Admin => {
            let result = db.usermanager.get_user_from_token(&input.token).await;
            if result.is_ok() && result.as_ref().unwrap().is_none() {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "[{}]: Token not related to any account.",
                            input.token
                        )),
                    ),
                });
            }
            let mut tokens = result.unwrap().unwrap().clone().tokens.unwrap();
            tokens.retain(|filter| filter.token == input.token);
            let token = tokens.get(0).unwrap();
            return Ok(Json(account_model::AccountTokenInfos {
                token: token.token.clone(),
                timestamp: token.timestamp,
                expiration_timestamp: token.expiration_timestamp,
            }));
        }
        _ => {
            let result = result.unwrap().unwrap().tokens;
            if result.is_none() {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "[{}]: Token not related to any account.",
                            input.token
                        )),
                    ),
                });
            }
            let mut tokens = result.unwrap().clone();
            tokens.retain(|filter| filter.token == input.token);
            if tokens.is_empty() {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "[{}]: Token not related to any account.",
                            input.token
                        )),
                    ),
                });
            }
            let token = tokens.get(0).unwrap();
            return Ok(Json(account_model::AccountTokenInfos {
                token: token.token.clone(),
                timestamp: token.timestamp,
                expiration_timestamp: token.expiration_timestamp,
            }));
        }
    }
}

#[post("/user/delete", data = "<input>")]
pub async fn delete(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountToken>,
) -> Result<Json<String>, account_errors::Error> {
    let api = api.apiuser;

    let result = db.usermanager.get_user(None, Some(&api.uuid)).await;
    if result.is_ok() && result.as_ref().unwrap().is_none() {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(
                400,
                Some(format!("[{}]: User account doesn't exist.", api.uuid)),
            ),
        });
    }
    match api.access.role {
        ApiUserRoleType::Admin => {
            let result = db.usermanager.get_user_from_token(&input.token).await;
            if result.is_ok() && result.as_ref().unwrap().is_none() {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "[{}]: Token not related to any account.",
                            input.token
                        )),
                    ),
                });
            }
            match db
                .apiusermanager
                .delete_apiuser_from_token(&input.token)
                .await
            {
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
        _ => {
            let result = result.unwrap().unwrap().tokens;
            if result.is_none() {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "[{}]: Token not related to any account.",
                            input.token
                        )),
                    ),
                });
            }
            let mut tokens = result.unwrap().clone();
            tokens.retain(|filter| filter.token == input.token);
            if tokens.is_empty() {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "[{}]: Token not related to any account.",
                            input.token
                        )),
                    ),
                });
            }
            match db
                .apiusermanager
                .delete_apiuser_from_token(&input.token)
                .await
            {
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
    }
}

#[post("/user/clear-tokens", data = "<input>")]
pub async fn clear_tokens(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountToken>,
) -> Result<Json<String>, account_errors::Error> {
    let api = api.apiuser;

    let result = db.usermanager.get_user(None, Some(&api.uuid)).await;
    if result.is_ok() && result.as_ref().unwrap().is_none() {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(
                400,
                Some(format!("[{}]: User account doesn't exist.", api.uuid)),
            ),
        });
    }
    match api.access.role {
        ApiUserRoleType::Admin => {
            let result = db.usermanager.get_user_from_token(&input.token).await;
            if result.is_ok() && result.as_ref().unwrap().is_none() {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "[{}]: Token not related to any account.",
                            input.token
                        )),
                    ),
                });
            }
            match db.apiusermanager.clear_tokens(&input.token).await {
                Ok(_) => {
                    return Ok(Json("Tokens removed.".to_string()));
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
        _ => {
            let result = result.unwrap().unwrap().tokens;
            if result.is_none() {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "[{}]: Token not related to any account.",
                            input.token
                        )),
                    ),
                });
            }
            let mut tokens = result.unwrap().clone();
            tokens.retain(|filter| filter.token == input.token);
            if tokens.is_empty() {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "[{}]: Token not related to any account.",
                            input.token
                        )),
                    ),
                });
            }
            match db.apiusermanager.clear_tokens(&input.token).await {
                Ok(_) => {
                    return Ok(Json("Tokens removed.".to_string()));
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
    }
}
