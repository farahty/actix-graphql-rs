use async_graphql::{InputObject, SimpleObject};
use bson::Document;

use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    Collection, Database,
};
use serde::{Deserialize, Serialize};

use crate::db::repo::MongoRepository;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub email: String,
    pub created_at: Option<DateTime>,
}

#[derive(SimpleObject, Clone)]
pub struct UserGQL {
    pub id: Option<String>,
    pub username: String,
    pub email: String,
    pub created_at: Option<String>,
}

impl From<User> for UserGQL {
    fn from(user: User) -> Self {
        UserGQL {
            id: user.id.map(|oid| oid.to_hex()),
            username: user.username,
            email: user.email,
            created_at: user.created_at.map(|dt| dt.to_string()),
        }
    }
}

#[derive(InputObject)]
pub struct NewUserInput {
    pub username: String,
    pub email: String,
}

#[derive(Clone)]
pub struct UserRepository {
    collection: Collection<Document>,
}

impl UserRepository {
    /// Initialize from a shared MongoDB database
    pub fn new(db: &Database) -> Self {
        let collection = db.collection::<Document>("users");
        Self { collection }
    }
}

impl MongoRepository<User, NewUserInput> for UserRepository {
    fn collection(&self) -> &Collection<Document> {
        &self.collection
    }

    fn to_entity(&self, input: &NewUserInput) -> User {
        User {
            id: None,
            username: input.username.clone(),
            email: input.email.clone(),
            created_at: Some(bson::DateTime::now()),
        }
    }
}
