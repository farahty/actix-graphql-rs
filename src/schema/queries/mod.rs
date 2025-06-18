mod categories;
mod todos;
mod users;
use async_graphql::MergedObject;

#[derive(MergedObject, Default)]
pub struct Query(
    pub users::UsersQueries,
    pub categories::CategoriesQueries,
    todos::TodosQueries,
);
