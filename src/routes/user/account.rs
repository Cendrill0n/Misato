use rocket::serde::json::Json;
use rocket::*;

use misato::models::account_model;

use misato_database::database::*;

use misato_database::models::apiuser_model;

use crate::errors::account_errors;

use crate::fairings::authentication::ApiToken;

#[post("/user/check-token", data = "<input>")]
pub async fn check_token(
    api: ApiToken,
    db: &State<Database>,
    input: Json<account_model::AccountToken>,
) -> Result<Json<account_model::Account>, account_errors::Error> {
    if api.apiuser.access.role != apiuser_model::ApiUserRoleType::Dev
        && api.apiuser.access.role != apiuser_model::ApiUserRoleType::Admin
    {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(400, Some("No permission".to_string())),
        });
    }
    match db.usermanager.get_user_from_token(&input.token).await {
        Ok(user) => match user {
            Some(user) => {
                return Ok(Json(account_model::Account {
                    username: user.username,
                    uuid: user.uuid,
                    token: input.token.to_string(),
                }))
            }
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

#[post("/user/delete", data = "<input>")]
pub async fn delete(
    api: ApiToken,
    db: &State<Database>,
    input: Json<account_model::AccountFilter>,
) -> Result<Json<String>, account_errors::Error> {
    match db
        .usermanager
        .delete_user(input.username.clone(), input.uuid.clone())
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
                        Some(format!(
                            "Account doesn't exist with the uuid: [{}] or the username: [{}]",
                            input
                                .username
                                .as_ref()
                                .unwrap_or(&"none_provided".to_string()),
                            input.uuid.as_ref().unwrap_or(&"none_provided".to_string()),
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

#[post("/user/clear-tokens", data = "<input>")]
pub async fn clear_tokens(
    api: ApiToken,
    db: &State<Database>,
    input: Json<account_model::AccountToken>,
) -> Result<Json<String>, account_errors::Error> {
    match db.usermanager.clear_tokens_from_token(&input.token).await {
        Ok(user) => match user.modified_count {
            1 => return Ok(Json("tokens cleared.".to_string())),
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
