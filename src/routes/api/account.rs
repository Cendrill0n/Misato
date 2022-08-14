use rocket::serde::json::Json;
use rocket::*;

use misato_database::{database::*, models::*};
use misato_security::password::*;

use misato::models::account_model;

use crate::errors::account_errors;
use crate::fairings::authentication::ApiToken;

const TOKEN_DURATION: u64 = 24 * 60 * 60;

#[post("/api/signup", data = "<input>")]
pub async fn signup(
    api: ApiToken,
    db: &State<Database>,
    input: Json<account_model::AccountCredentials>,
) -> Result<Json<account_model::Account>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Admin
        || api.apiuser.token.is_none()
    {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission".to_string())),
        });
    }
    let mut user = apiuser_model::ApiUser::create(
        input.username.to_string(),
        Password::hash_password(input.password.as_bytes()),
        None,
    );

    match db.apiusermanager.username_exists(&user.username).await {
        Ok(exists) => {
            if exists {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "Account already exists with the username: [{}]",
                            input.username
                        )),
                    ),
                });
            }
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

    match db.apiusermanager.create_apiuser(&user).await {
        Ok(_) => {
            let token = user.new_token(TOKEN_DURATION);
            let _ = db.apiusermanager.set_token(&user.uuid, &token).await;
            return Ok(Json(account_model::Account {
                username: user.username,
                uuid: user.uuid,
                token: token.token.to_string(),
            }));
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

#[post("/api/login", data = "<input>")]
pub async fn login(
    db: &State<Database>,
    input: Json<account_model::AccountCredentials>,
) -> Result<Json<account_model::Account>, account_errors::Error> {
    match db
        .apiusermanager
        .get_apiuser(Some(&input.username), None)
        .await
    {
        Ok(mut user) => match &mut user {
            Some(user) if user.password.is_correct_password(input.password.as_bytes()) => {
                let token = user.new_token(TOKEN_DURATION);
                let _ = db.apiusermanager.set_token(&user.uuid, &token).await;
                return Ok(Json(account_model::Account {
                    username: user.username.clone(),
                    uuid: user.uuid.clone(),
                    token: token.token.to_string(),
                }));
            }
            _ => {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "Account doesn't exist with the username: [{}]",
                            input.username
                        )),
                    ),
                })
            }
        },
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

#[post("/api/check-token")]
pub async fn check_token(
    api: ApiToken,
    db: &State<Database>,
) -> Result<Json<account_model::Account>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Dev || api.apiuser.token.is_none()
    {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission".to_string())),
        });
    }
    let token = api.apiuser.token.unwrap().token;
    match db.apiusermanager.get_apiuser_from_token(&token).await {
        Ok(user) => match user {
            Some(user) => {
                return Ok(Json(account_model::Account {
                    username: user.username,
                    uuid: user.uuid,
                    token: token.to_string(),
                }))
            }
            _ => {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!("Account doesn't exist with the token: [{}]", token)),
                    ),
                })
            }
        },
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

#[post("/api/delete")]
pub async fn delete(
    api: ApiToken,
    db: &State<Database>,
) -> Result<Json<String>, account_errors::Error> {
    if api.apiuser.token.is_none() {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission".to_string())),
        });
    }
    let token = api.apiuser.token.unwrap().token;
    match db.apiusermanager.delete_apiuser_from_token(&token).await {
        Ok(user) => match user {
            Some(count) if count.deleted_count >= 1 => {
                return Ok(Json("account deleted.".to_string()));
            }
            _ => {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!("Account doesn't exist with the token: [{}]", token)),
                    ),
                })
            }
        },
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

#[post("/api/clear-tokens", data = "<input>")]
pub async fn clear_tokens(
    db: &State<Database>,
    input: Json<account_model::AccountToken>,
) -> Result<Json<String>, account_errors::Error> {
    match db
        .apiusermanager
        .clear_tokens_from_token(&input.token)
        .await
    {
        Ok(user) => match user.modified_count {
            1 => return Ok(Json("token removed.".to_string())),
            _ => {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "Account doesn't exist with the token: [{}]",
                            input.token
                        )),
                    ),
                })
            }
        },
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
