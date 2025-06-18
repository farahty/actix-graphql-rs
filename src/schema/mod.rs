mod mutations;
mod queries;
mod subscriptions;

use crate::models::categories::CategoryRepository;
use crate::models::todos::TodoRepository;
use crate::models::users::UserRepository;
use mongodb::Database;
use redis::aio::ConnectionManager;
use std::sync::Arc;

pub type Schema =
    async_graphql::Schema<queries::Query, mutations::Mutation, subscriptions::Subscription>;

pub fn build_schema(redis: Arc<ConnectionManager>, db: &Database) -> Schema {
    let user_repo = Arc::new(UserRepository::new(&db));
    let category_repo = Arc::new(CategoryRepository::new(&db));
    let todo_repo = Arc::new(TodoRepository::new(&db));

    async_graphql::Schema::build(
        queries::Query::default(),
        mutations::Mutation::default(),
        subscriptions::Subscription::default(),
    )
    .data(redis)
    .data(user_repo)
    .data(category_repo)
    .data(todo_repo)
    .finish()
}
