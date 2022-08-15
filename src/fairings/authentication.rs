use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};

use misato_database::{database::*, models::*};

pub struct UserToken {
    pub user: user_model::User,
}

#[derive(Debug)]
pub enum UserTokenError {
    BadCount,
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserToken {
    type Error = UserTokenError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<UserToken, Self::Error> {
        let keys: Vec<_> = request.headers().get("X-Misato-Token").collect();
        match keys.len() {
            0 => return Outcome::Failure((Status::BadRequest, UserTokenError::Missing)),
            1 => {
                let token = keys.get(0).unwrap();

                let db = request.rocket().state::<Database>().unwrap();

                let user = db.usermanager.get_user_from_token(&token).await;

                if user.is_ok() && user.as_ref().unwrap().is_some() {
                    return Outcome::Success(UserToken {
                        user: user.unwrap().unwrap(),
                    });
                }
                return Outcome::Failure((Status::BadRequest, UserTokenError::Invalid));
            }
            _ => {
                return Outcome::Failure((Status::BadRequest, UserTokenError::BadCount));
            }
        }
    }
}
