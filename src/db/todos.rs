use async_graphql::{InputObject, SimpleObject};
use bson::{oid::ObjectId, DateTime, Document};
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

use crate::db::repo::MongoRepository;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub text: String,
    pub completed: bool,
    pub created_at: Option<DateTime>,
}

#[derive(SimpleObject, Clone)]
pub struct TodoGQL {
    pub id: Option<String>,
    pub text: String,
    pub completed: bool,
    pub created_at: Option<String>,
}

impl From<Todo> for TodoGQL {
    fn from(value: Todo) -> Self {
        TodoGQL {
            id: value.id.map(|oid| oid.to_hex()),
            text: value.text,
            completed: value.completed,
            created_at: value.created_at.map(|dt| dt.to_string()),
        }
    }
}

impl From<TodoGQL> for Todo {
    fn from(value: TodoGQL) -> Self {
        Todo {
            id: value.id.and_then(|id| ObjectId::parse_str(id).ok()),
            text: value.text,
            completed: value.completed,
            created_at: value
                .created_at
                .and_then(|dt| DateTime::parse_rfc3339_str(dt).ok()),
        }
    }
}

#[derive(InputObject)]
pub struct NewTodoInput {
    pub text: String,
    pub completed: Option<bool>,
}

#[derive(Clone)]
pub struct TodoRepository {
    collection: Collection<Document>,
}

impl TodoRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection::<Document>("todos");
        Self { collection }
    }
}

impl MongoRepository<Todo, NewTodoInput> for TodoRepository {
    fn collection(&self) -> &Collection<Document> {
        &self.collection
    }

    fn to_entity(&self, input: &NewTodoInput) -> Todo {
        Todo {
            id: None,
            text: input.text.clone(),
            completed: input.completed.unwrap_or(false),
            created_at: Some(DateTime::now()),
        }
    }
}
