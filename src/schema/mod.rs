mod categories;
mod todos;
mod users;

use crate::models::categories::CategoryRepository;
use crate::models::todos::TodoRepository;
use crate::models::users::UserRepository;
use async_graphql::{EmptySubscription, MergedObject};
use mongodb::Database;
use redis::aio::ConnectionManager;
use std::sync::Arc;

#[derive(MergedObject, Default)]
pub struct Query(
    users::UsersQueries,
    categories::CategoriesQueries,
    todos::TodosQueries,
);

#[derive(MergedObject, Default)]
pub struct Mutation(
    users::UsersMutations,
    categories::CategoriesMutations,
    todos::TodosMutations,
);

pub type Subscription = EmptySubscription;

pub type Schema = async_graphql::Schema<Query, Mutation, Subscription>;

pub fn build_schema(redis: Arc<ConnectionManager>, db: &Database) -> Schema {
    let user_repo = Arc::new(UserRepository::new(&db));
    let category_repo = Arc::new(CategoryRepository::new(&db));
    let todo_repo = Arc::new(TodoRepository::new(&db));

    async_graphql::Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(redis)
    .data(user_repo)
    .data(category_repo)
    .data(todo_repo)
    .finish()
}
