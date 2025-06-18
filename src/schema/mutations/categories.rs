use crate::models::categories::{CategoryGQL, CategoryRepository, NewCategoryInput};
use crate::db::repo::MongoRepository;
use async_graphql::{Context, Object};
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct CategoriesMutations;

#[Object]
impl CategoriesMutations {
    pub async fn create_category(
        &self,
        ctx: &Context<'_>,
        input: NewCategoryInput,
    ) -> Option<CategoryGQL> {
        let repo = ctx
            .data::<Arc<CategoryRepository>>()
            .expect("CategoryRepository not in context");
        match repo.create(&input).await {
            Ok(category) => Some(CategoryGQL::from(category)),
            Err(_) => None,
        }
    }
}
