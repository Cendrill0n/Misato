use rocket::serde::json::Json;
use rocket::*;

use misato::models::account_model;

use misato_database::database::*;

use misato_database::models::user_model;

use crate::errors::account_errors;

use crate::fairings::api_authentication::ApiUserToken;

async fn get_user(
    api: ApiUserToken,
    db: &State<Database>,
    token: &String,
) -> Result<user_model::User, account_errors::Error> {
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
    let user = result.unwrap().unwrap();
    if user.tokens.is_none() {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(
                400,
                Some(format!("[{}]: Token not related to any account.", token)),
            ),
        });
    }
    let mut tokens = user.tokens.clone().unwrap();
    tokens.retain(|filter| &filter.token == token);
    if tokens.is_empty() {
        return Err(account_errors::Error {
            content: account_model::AccountError::build(
                400,
                Some(format!("[{}]: Token not related to any account.", token)),
            ),
        });
    }
    Ok(user)
}

#[post("/user/check-token", data = "<input>")]
pub async fn check_token(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountToken>,
) -> Result<Json<account_model::AccountTokenInfos>, account_errors::Error> {
    match get_user(api, db, &input.token).await {
        Ok(user) => {
            let mut tokens = user.tokens.clone().unwrap();
            tokens.retain(|filter| filter.token == input.token);
            let token = tokens.get(0).unwrap();
            return Ok(Json(account_model::AccountTokenInfos {
                token: token.token.clone(),
                timestamp: token.timestamp,
                expiration_timestamp: token.expiration_timestamp,
                uuid: user.uuid,
            }));
        }
        Err(err) => return Err(err),
    }
}

#[post("/user/delete", data = "<input>")]
pub async fn delete(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountToken>,
) -> Result<Json<String>, account_errors::Error> {
    match get_user(api, db, &input.token).await {
        Ok(user) => {
            match db
                .apiusermanager
                .delete_apiuser_from_token(&input.token)
                .await
            {
                Ok(_) => {
                    return Ok(Json(format!("[{}]: Account deleted.", user.uuid)));
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
        Err(err) => return Err(err),
    }
}

#[post("/user/clear-tokens", data = "<input>")]
pub async fn clear_tokens(
    api: ApiUserToken,
    db: &State<Database>,
    input: Json<account_model::AccountToken>,
) -> Result<Json<String>, account_errors::Error> {
    match get_user(api, db, &input.token).await {
        Ok(_) => match db.apiusermanager.clear_tokens(&input.token).await {
            Ok(_) => {
                return Ok(Json(format!("[{}]: Tokens removed.", input.token)));
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
        },
        Err(err) => return Err(err),
    }
}
