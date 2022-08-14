use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};

use misato_database::{database::*, models::*};

pub struct ApiToken {
    pub apiuser: apiuser_model::ApiUser,
}

#[derive(Debug)]
pub enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiToken {
    type Error = ApiKeyError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<ApiToken, Self::Error> {
        let keys: Vec<_> = request.headers().get("X-Misato-Token").collect();
        match keys.len() {
            0 => return Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 => {
                let token = keys.get(0).unwrap();

                let db = request.rocket().state::<Database>().unwrap();

                let apiuser = db.apiusermanager.get_apiuser_from_token(&token).await;

                if apiuser.is_ok() && apiuser.as_ref().unwrap().is_some() {
                    return Outcome::Success(ApiToken {
                        apiuser: apiuser.unwrap().unwrap(),
                    });
                }
                return Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid));
            }
            _ => {
                return Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount));
            }
        }
    }
}
