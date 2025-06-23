use async_graphql::{Context, Object, Result};
use mongodb::Database;
use redis::AsyncCommands;

use crate::{
    db::repo::MongoRepository,
    models::todos::{NewTodoInput, TodoRepository},
    schema::todos::TodoGQL,
};

#[derive(Default, Clone)]
pub struct TodosQueries;

#[Object]
impl TodosQueries {
    pub async fn todos(&self, ctx: &Context<'_>) -> Result<Vec<TodoGQL>> {
        let repo = TodoRepository::new(ctx.data::<Database>()?);
        let todos = repo.find_all().await?;
        Ok(todos.into_iter().map(TodoGQL::from).collect())
    }

    pub async fn ping(&self, ctx: &Context<'_>) -> Result<String> {
        let mut redis = ctx.data::<redis::aio::ConnectionManager>()?.clone();

        let _: () = redis.set("ping", "pong").await?;
        let pong: String = redis.get("ping").await?;
        Ok(pong)
    }
}

#[derive(Default)]
pub struct TodosMutations;

#[Object]
impl TodosMutations {
    pub async fn create_todo(&self, ctx: &Context<'_>, input: NewTodoInput) -> Result<TodoGQL> {
        let repo = TodoRepository::new(ctx.data::<Database>()?);
        let todo = repo.create(&input).await?;
        Ok(todo.into())
    }
}
