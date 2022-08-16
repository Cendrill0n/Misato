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
) -> Result<Json<account_model::AccountTokenInfos>, account_errors::Error> {
    match db.usermanager.get_user(Some(&input.username), None).await {
        Ok(mut user) => match &mut user {
            Some(user) => {
                let password = user.password.as_ref();
                if password.is_some()
                    && password
                        .unwrap()
                        .is_correct_password(input.password.as_bytes())
                {
                    let token = user.new_token(TOKEN_DURATION);
                    let _ = db.usermanager.save_token(&user.uuid, &token).await;
                    return Ok(Json(account_model::AccountTokenInfos {
                        token: token.token,
                        timestamp: token.timestamp,
                        expiration_timestamp: token.expiration_timestamp,
                        uuid: user.uuid.clone(),
                    }));
                } else {
                    return Err(account_errors::Error {
                        content: account_model::AccountError::build(
                            400,
                            Some(format!("[{}]: Account has disabled login.", input.username)),
                        ),
                    });
                }
            }
            _ => {
                return Err(account_errors::Error {
                    content: account_model::AccountError::build(
                        400,
                        Some(format!(
                            "[{}]: Username not related to any account.",
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
