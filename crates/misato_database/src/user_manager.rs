use mongodb::{bson::doc, error::Error, results::InsertOneResult, sync::Collection};
use uuid::Uuid;

use crate::models::user_model::*;

pub struct UserManager {
    pub users: Collection<User>,
}

impl UserManager {
    pub fn init(users: Collection<User>) -> Self {
        UserManager { users }
    }

    pub fn create_user(&self, user: &User) -> Result<InsertOneResult, Error> {
        let target = self
            .users
            .insert_one(user, None)
            .ok()
            .expect("Error whilst creating user.");
        Ok(target)
    }

    pub fn get_user(
        &self,
        username: Option<String>,
        uuid: Option<Uuid>,
    ) -> Result<Option<User>, Error> {
        if uuid.is_some() {
            match self
                .users
                .find_one(doc! {"uuid": uuid.unwrap().to_string() }, None)
            {
                Ok(Some(user)) => return Ok(Some(user)),
                _ => {}
            }
        }
        match username {
            Some(username) => match self.users.find_one(doc! {"username": username }, None) {
                Ok(Some(user)) => return Ok(Some(user)),
                Err(error) => return Err(error),
                _ => {}
            },
            None => {}
        }
        Ok(None)
    }

    pub fn delete_user(&self, _user: User) {}
}
