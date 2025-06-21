use std::sync::Arc;

use async_graphql::{ComplexObject, Context};
use bson::doc;

use crate::{
    db::repo::MongoRepository,
    models::{todos::Todo, users::UserRepository},
    schema::users::UserGQL,
};

#[derive(async_graphql::SimpleObject, Clone, Debug)]
#[graphql(complex)]
pub struct TodoGQL {
    pub id: Option<String>,
    pub text: String,
    pub completed: bool,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<Todo> for TodoGQL {
    fn from(value: Todo) -> Self {
        TodoGQL {
            id: value.id.as_ref().map(|oid| oid.to_hex()),
            text: value.text.clone(),
            completed: value.completed,
            description: value.description.clone(),
            created_at: value.created_at.map(|dt| dt.to_string()),
            updated_at: value.updated_at.map(|dt| dt.to_string()),
        }
    }
}

#[ComplexObject]
impl TodoGQL {
    async fn created_by(&self, ctx: &Context<'_>) -> crate::error::Result<UserGQL> {
        let repo = ctx.data::<Arc<UserRepository>>()?;
        let user = repo.find_one(doc! {}).await?;

        match user {
            Some(user) => Ok(user.into()),
            None => Err(async_graphql::Error::new("User not found")),
        }
    }
}
