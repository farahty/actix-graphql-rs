use crate::db::repo::MongoRepository;
use crate::models::todos::NewTodoInput;
use crate::models::todos::{TodoGQL, TodoRepository};
use async_graphql::{Context, Object, Result};
use std::sync::Arc;

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

#[derive(Default)]
pub struct TodosMutations {}

#[Object]
impl TodosMutations {
    pub async fn create_todo(&self, ctx: &Context<'_>, input: NewTodoInput) -> Result<TodoGQL> {
        let repo = ctx.data::<Arc<TodoRepository>>()?;
        let todo = repo.create(&input).await?;
        Ok(TodoGQL::from(todo))
    }
}
