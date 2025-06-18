use crate::db::repo::MongoRepository;
use async_graphql::{InputObject, SimpleObject};
use bson::Document;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    Collection, Database,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime>,
}

#[derive(SimpleObject, Clone)]
pub struct CategoryGQL {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
}

impl From<Category> for CategoryGQL {
    fn from(category: Category) -> Self {
        CategoryGQL {
            id: category.id.map(|oid| oid.to_hex()),
            name: category.name,
            description: category.description,
            created_at: category.created_at.map(|dt| dt.to_string()),
        }
    }
}

#[derive(InputObject)]
pub struct NewCategoryInput {
    pub name: String,
    pub description: Option<String>,
}

impl Category {
    pub fn from_new(input: &NewCategoryInput) -> Self {
        Self {
            id: None,
            name: input.name.clone(),
            description: input.description.clone(),
            created_at: Some(DateTime::now()),
        }
    }
}

#[derive(Clone)]
pub struct CategoryRepository {
    collection: Collection<Document>,
}

impl CategoryRepository {
    /// Initialize the repo from a shared database
    pub fn new(db: &Database) -> Self {
        let collection = db.collection::<Document>("categories");
        Self { collection }
    }
}

impl MongoRepository<Category, NewCategoryInput> for CategoryRepository {
    fn collection(&self) -> &Collection<Document> {
        &self.collection
    }

    fn to_entity(&self, input: &NewCategoryInput) -> Category {
        Category {
            id: None,
            name: input.name.clone(),
            description: input.description.clone(),
            created_at: None,
        }
    }
}
