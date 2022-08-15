use mongodb::{
    bson::{doc, Document},
    error::Error,
    options::ReplaceOptions,
    results::{DeleteResult, UpdateResult},
    Collection,
};

use misato_utils::get_current_timestamp;

use crate::models::apiuser_model::*;

pub struct ApiUserManager {
    pub apiusers: Collection<ApiUser>,
}

impl ApiUserManager {
    pub fn init(apiusers: Collection<ApiUser>) -> Self {
        Self { apiusers }
    }

    pub async fn username_exists(&self, username: &str) -> Result<bool, Error> {
        Ok(self
            .apiusers
            .count_documents(doc! { "username": username }, None)
            .await?
            != 0)
    }

    pub async fn create_apiuser(&self, apiuser: &ApiUser) -> Result<UpdateResult, Error> {
        let target = self
            .apiusers
            .replace_one(
                doc! { "username": apiuser.username.clone() },
                apiuser,
                ReplaceOptions::builder().upsert(true).build(),
            )
            .await?;
        Ok(target)
    }

    pub async fn get_apiuser(
        &self,
        username: Option<&str>,
        uuid: Option<&str>,
    ) -> Result<Option<ApiUser>, Error> {
        let mut doc: Document = Document::new();
        if uuid.is_some() {
            doc = doc! {"uuid": uuid.unwrap()};
        }
        if username.is_some() {
            doc = doc! {"username": username.unwrap()};
        }
        if doc.is_empty() {
            return Ok(None);
        }
        match self.apiusers.find_one(doc, None).await? {
            Some(apiuser) => Ok(Some(apiuser)),
            None => Ok(None),
        }
    }

    pub async fn delete_apiuser(
        &self,
        username: Option<&str>,
        uuid: Option<&str>,
    ) -> Result<Option<DeleteResult>, Error> {
        let mut doc: Document = Document::new();
        if uuid.is_some() {
            doc = doc! {"uuid": uuid.unwrap()};
        }
        if username.is_some() {
            doc = doc! {"username": username.unwrap()};
        }
        if doc.is_empty() {
            return Ok(None);
        }
        Ok(Some(self.apiusers.delete_one(doc, None).await?))
    }

    pub async fn delete_apiuser_from_token(
        &self,
        token: &str,
    ) -> Result<Option<DeleteResult>, Error> {
        Ok(Some(
            self.apiusers
                .delete_one(doc! {"token.token": token, "token.expiration_timestamp": { "$gte": get_current_timestamp() as i64 } }, None)
                .await?,
        ))
    }

    pub async fn set_token(&self, uuid: &str, token: &ApiUserToken) -> Result<UpdateResult, Error> {
        let doc = mongodb::bson::to_document(token).unwrap();
        let update = doc! {"$set": {"token": doc} };
        Ok(self
            .apiusers
            .update_one(doc! {"uuid": uuid}, update, None)
            .await?)
    }

    pub async fn clear_tokens(&self, uuid: &str) -> Result<UpdateResult, Error> {
        let update = doc! {"$unset": {"token": ""} };
        Ok(self
            .apiusers
            .update_one(doc! {"uuid": uuid}, update, None)
            .await?)
    }

    pub async fn clear_tokens_from_token(&self, token: &str) -> Result<UpdateResult, Error> {
        let update = doc! {"$unset": {"token": ""} };
        Ok(self
            .apiusers
            .update_one(doc! {"token.token": token, "token.expiration_timestamp": { "$gte": get_current_timestamp() as i64 } }, update, None)
            .await?)
    }

    pub async fn get_apiuser_from_token(&self, token: &str) -> Result<Option<ApiUser>, Error> {
        match self
            .apiusers
            .find_one(
                doc! {"token.token": token, "token.expiration_timestamp": { "$gte": get_current_timestamp() as i64 } },
                None,
            )
            .await?
        {
            Some(apiuser) => Ok(Some(apiuser)),
            None => Ok(None),
        }
    }
}
