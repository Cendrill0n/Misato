use mongodb::{
    bson::{doc, Document},
    error::Error,
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Collection,
};

use misato_utils::get_current_timestamp;

use crate::models::user_model::*;

pub struct UserManager {
    pub users: Collection<User>,
}

impl UserManager {
    pub fn init(users: Collection<User>) -> Self {
        Self { users }
    }

    pub async fn username_exists(&self, username: &str) -> Result<bool, Error> {
        Ok(self
            .users
            .count_documents(doc! { "username": username }, None)
            .await?
            != 0)
    }

    pub async fn uuid_exists(&self, uuid: &str) -> Result<bool, Error> {
        Ok(self
            .users
            .count_documents(doc! { "uuid": uuid }, None)
            .await?
            != 0)
    }

    pub async fn create_user(&self, user: &User) -> Result<InsertOneResult, Error> {
        let target = self.users.insert_one(user, None).await?;
        Ok(target)
    }

    pub async fn get_user(
        &self,
        username: Option<&str>,
        uuid: Option<&str>,
    ) -> Result<Option<User>, Error> {
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
        match self.users.find_one(doc, None).await? {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    }

    pub async fn delete_user(
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
        Ok(Some(self.users.delete_one(doc, None).await?))
    }

    pub async fn delete_user_from_token(&self, token: &str) -> Result<Option<DeleteResult>, Error> {
        Ok(Some(
            self.users
                .delete_one(doc! {"tokens.token": token, "tokens.expiration_timestamp": { "$gte": get_current_timestamp() as i64 } }, None)
                .await?,
        ))
    }

    pub async fn save_token(&self, uuid: &str, token: &UserToken) -> Result<UpdateResult, Error> {
        let doc = mongodb::bson::to_document(token).unwrap();
        let update = doc! {"$push": {"tokens": doc} };
        Ok(self
            .users
            .update_one(doc! {"uuid": uuid}, update, None)
            .await?)
    }

    pub async fn clear_tokens(&self, uuid: &str) -> Result<UpdateResult, Error> {
        let update = doc! {"$unset": {"tokens": ""} };
        Ok(self
            .users
            .update_one(doc! {"uuid": uuid}, update, None)
            .await?)
    }

    pub async fn clear_tokens_from_token(&self, token: &str) -> Result<UpdateResult, Error> {
        let update = doc! {"$unset": {"tokens": ""} };
        Ok(self
            .users
            .update_one(doc! {"tokens.token": token, "tokens.expiration_timestamp": { "$gte": get_current_timestamp() as i64 } }, update, None)
            .await?)
    }

    pub async fn get_user_from_token(&self, token: &str) -> Result<Option<User>, Error> {
        match self
            .users
            .find_one(
                doc! {"tokens.token": token, "tokens.expiration_timestamp": { "$gte": get_current_timestamp() as i64 } },
                None,
            )
            .await?
        {
            Some(user) => Ok(Some(user)),
            None => Ok(None),
        }
    }
}
