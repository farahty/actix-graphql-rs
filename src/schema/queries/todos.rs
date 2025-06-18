use std::sync::Arc;

use async_graphql::{Context, Object, Result};

use crate::db::{
    repo::MongoRepository,
    todos::{TodoGQL, TodoRepository},
};

#[derive(Default, Clone)]
pub struct TodosQueries;

#[Object]
impl TodosQueries {
    pub async fn todos(&self, ctx: &Context<'_>) -> Result<Vec<TodoGQL>> {
        let repo = ctx.data::<Arc<TodoRepository>>()?;
        let todos = repo.find_all().await?;
        Ok(todos.into_iter().map(TodoGQL::from).collect())
    }
}
