use rocket::serde::json::Json;
use rocket::*;

use misato_database::{database::*, models::*};
use misato_security::password::*;

use misato::models::account_model;

use crate::errors::account_errors;

const TOKEN_DURATION: u64 = 24 * 60 * 60;

use crate::fairings::api_authentication::ApiUserToken;

#[post("/admin/signup", data = "<input>")]
pub async fn signup(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountCredentials>,
) -> Result<Json<account_model::AccountTokenInfos>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Admin {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission.".to_string())),
        });
    }
    let mut user = user_model::User::create(
        input.username.to_string(),
        Password::hash_password(input.password.as_bytes()),
        None,
    );

    match db.usermanager.username_exists(&user.username).await {
        Ok(exists) => {
            if exists {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "[{}]: Username already used by an account.",
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

    match db.usermanager.create_user(&user).await {
        Ok(_) => {
            let token = user.new_token(TOKEN_DURATION);
            let _ = db.usermanager.save_token(&user.uuid, &token).await;
            return Ok(Json(account_model::AccountTokenInfos {
                token: token.token.clone(),
                timestamp: token.timestamp,
                expiration_timestamp: token.expiration_timestamp,
                uuid: user.uuid,
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

#[post("/admin/profile", data = "<input>")]
pub async fn profile(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountUuid>,
) -> Result<Json<account_model::Account>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Admin {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission.".to_string())),
        });
    }
    match db.usermanager.get_user(None, Some(&input.uuid)).await {
        Ok(user) => match user {
            Some(user) => {
                return Ok(Json(account_model::Account {
                    uuid: user.uuid.clone(),
                    username: user.username.clone(),
                }));
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

#[post("/admin/profile-from-token", data = "<input>")]
pub async fn profile_from_token(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountToken>,
) -> Result<Json<account_model::Account>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Admin {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission.".to_string())),
        });
    }
    match db.usermanager.get_user_from_token(&input.token).await {
        Ok(user) => match user {
            Some(user) => {
                return Ok(Json(account_model::Account {
                    uuid: user.uuid.clone(),
                    username: user.username.clone(),
                }));
            }
            _ => {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "[{}]: Token not related to any account.",
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

#[post("/admin/refresh-token", data = "<input>")]
pub async fn refresh_token(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountUuid>,
) -> Result<Json<account_model::AccountTokenInfos>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Admin {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission.".to_string())),
        });
    }
    match db.usermanager.get_user(None, Some(&input.uuid)).await {
        Ok(mut user) => match &mut user {
            Some(user) => {
                let token = user.new_token(TOKEN_DURATION);
                let _ = db.usermanager.save_token(&user.uuid, &token).await;
                return Ok(Json(account_model::AccountTokenInfos {
                    token: token.token.clone(),
                    timestamp: token.timestamp,
                    expiration_timestamp: token.expiration_timestamp,
                    uuid: user.uuid.clone(),
                }));
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

#[post("/admin/check-token", data = "<input>")]
pub async fn check_token(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountToken>,
) -> Result<Json<account_model::AccountTokenInfos>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Admin {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission.".to_string())),
        });
    }
    match db.usermanager.get_user_from_token(&input.token).await {
        Ok(user) => match user {
            Some(user) => {
                let mut tokens = user.tokens.clone().unwrap();
                tokens.retain(|filter| filter.token == input.token);
                let token = &tokens.get(0).unwrap();
                return Ok(Json(account_model::AccountTokenInfos {
                    token: token.token.clone(),
                    timestamp: token.timestamp,
                    expiration_timestamp: token.expiration_timestamp,
                    uuid: user.uuid,
                }));
            }
            _ => {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "[{}]: Token not related to any account.",
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

#[post("/admin/delete", data = "<input>")]
pub async fn delete(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountUuid>,
) -> Result<Json<String>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Admin {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission.".to_string())),
        });
    }
    match db.usermanager.delete_user(None, Some(&input.uuid)).await {
        Ok(user) => match user {
            Some(count) if count.deleted_count >= 1 => {
                return Ok(Json("Account deleted.".to_string()));
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

#[post("/admin/clear-tokens", data = "<input>")]
pub async fn clear_tokens(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountUuid>,
) -> Result<Json<String>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Admin {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission.".to_string())),
        });
    }
    match db.usermanager.clear_tokens(&input.uuid).await {
        Ok(user) => match user.modified_count {
            1 => return Ok(Json("Tokens cleared.".to_string())),
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
