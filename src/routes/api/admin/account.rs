use rocket::serde::json::Json;
use rocket::*;

use misato_database::{database::*, models::*};

use misato::models::account_model;

use crate::errors::account_errors;
use crate::fairings::api_authentication::ApiUserToken;

const TOKEN_DURATION: u64 = 24 * 60 * 60;

#[post("/api/admin/signup", data = "<input>")]
pub async fn signup(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountUuid>,
) -> Result<Json<account_model::AccountTokenInfos>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Admin {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission".to_string())),
        });
    }
    let mut user = apiuser_model::ApiUser::create(input.uuid.clone());

    let result = db
        .apiusermanager
        .get_apiuser(None, Some(&user.uuid.to_string()))
        .await;
    if result.is_ok() && result.as_ref().unwrap().is_some() {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(
                400,
                Some(format!("[{}]: API Account already exists.", input.uuid)),
            ),
        });
    }
    if result.is_err() {
        println!("{:?}", result.unwrap_err());
        return Err(account_errors::Error {
            content: account_model::AccountError::build(500, Some("Database error.".to_string())),
        });
    }

    match db.apiusermanager.uuid_exists(&user.uuid).await {
        Ok(exists) => {
            if !exists {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!("[{}]: Account doesn't exist.", input.uuid)),
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

#[post("/api/admin/refresh-token", data = "<input>")]
pub async fn refresh_token(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountUuid>,
) -> Result<Json<account_model::AccountTokenInfos>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Admin {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission".to_string())),
        });
    }
    match db
        .apiusermanager
        .get_apiuser(None, Some(&input.uuid.to_string()))
        .await
    {
        Ok(mut user) => match &mut user {
            Some(user) => {
                let token = user.new_token(TOKEN_DURATION);
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
            _ => {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!("[{}]: Account doesn't exist.", input.uuid)),
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

#[post("/api/admin/check-token", data = "<input>")]
pub async fn check_token(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountToken>,
) -> Result<Json<account_model::AccountTokenInfos>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Admin {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission".to_string())),
        });
    }
    match db.apiusermanager.get_apiuser_from_token(&input.token).await {
        Ok(user) => match user {
            Some(user) => {
                let token = user.token.unwrap();
                return Ok(Json(account_model::AccountTokenInfos {
                    token: token.token,
                    timestamp: token.timestamp,
                    expiration_timestamp: token.expiration_timestamp,
                }));
            }
            _ => {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "[{}]: Token not related to any account.",
                            &input.token
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

#[post("/api/admin/delete", data = "<input>")]
pub async fn delete(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountUuid>,
) -> Result<Json<String>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Admin {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission".to_string())),
        });
    }
    match db
        .apiusermanager
        .delete_apiuser(None, Some(&input.uuid))
        .await
    {
        Ok(user) => match user {
            Some(count) if count.deleted_count >= 1 => {
                return Ok(Json("account deleted.".to_string()));
            }
            _ => {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!("[{}]: Account doesn't exist.", input.uuid)),
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

#[post("/api/admin/clear-tokens", data = "<input>")]
pub async fn clear_tokens(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountUuid>,
) -> Result<Json<String>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Admin {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission".to_string())),
        });
    }
    match db.apiusermanager.clear_tokens(&input.uuid).await {
        Ok(user) => match user.modified_count {
            1 => return Ok(Json("Token removed.".to_string())),
            _ => {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!("[{}]: Account doesn't exist.", input.uuid)),
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
