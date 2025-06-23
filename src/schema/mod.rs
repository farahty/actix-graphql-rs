mod categories;
mod todos;
mod users;

use async_graphql::{EmptySubscription, MergedObject};
use mongodb::Database;
use redis::aio::ConnectionManager;

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

pub fn build_schema(redis: ConnectionManager, db: Database) -> Schema {
    async_graphql::Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(redis)
    .data(db)
    .finish()
}
