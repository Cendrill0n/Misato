use rocket::serde::json::Json;
use rocket::*;

use misato::models::*;

use misato_database::database::*;

use crate::errors::account_errors;

const TOKEN_DURATION: u64 = 24 * 60 * 60;

#[post("/login", data = "<input>")]
pub async fn login(
    db: &State<Database>,
    input: Json<account_model::AccountCredentials>,
) -> Result<Json<account_model::Account>, account_errors::Error> {
    match db.usermanager.get_user(Some(&input.username), None).await {
        Ok(mut user) => match &mut user {
            Some(user) if user.password.is_correct_password(input.password.as_bytes()) => {
                let token = user.new_token(TOKEN_DURATION);
                let _ = db.usermanager.save_token(&user.uuid, &token).await;
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
