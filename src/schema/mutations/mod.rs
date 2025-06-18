mod categories;
mod todos;
mod users;
use async_graphql::MergedObject;
use categories::CategoriesMutations;
use users::UsersMutations;

use crate::schema::mutations::todos::TodosMutations;

#[derive(MergedObject, Default)]
pub struct Mutation(pub UsersMutations, pub CategoriesMutations, TodosMutations);
