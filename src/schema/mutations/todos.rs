use std::sync::Arc;

use async_graphql::{Context, Error, Object, Result};

use crate::db::repo::MongoRepository;
use crate::models::todos::{NewTodoInput, TodoGQL, TodoRepository};

#[derive(Default)]
pub struct TodosMutations {}

#[Object]
impl TodosMutations {
    pub async fn create_todo(&self, ctx: &Context<'_>, input: NewTodoInput) -> Result<TodoGQL> {
        let repo = ctx.data::<Arc<TodoRepository>>()?;

        match repo.create(&input).await {
            Ok(todo) => Ok(TodoGQL::from(todo)),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
}
