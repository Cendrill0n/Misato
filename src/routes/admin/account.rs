use rocket::serde::json::Json;
use rocket::*;

use misato_database::{database::*, models::*};
use misato_security::password::*;

use misato::models::account_model;

use crate::errors::account_errors;

const TOKEN_DURATION: u64 = 24 * 60 * 60;

use crate::fairings::authentication::ApiToken;

#[post("/admin/signup", data = "<input>")]
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

    match db.usermanager.create_user(&user).await {
        Ok(_) => {
            let token = user.new_token(TOKEN_DURATION);
            let _ = db.usermanager.save_token(&user.uuid, &token).await;
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
