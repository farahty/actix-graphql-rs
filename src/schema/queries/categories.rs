use crate::models::categories::{CategoryGQL, CategoryRepository};
use crate::db::repo::MongoRepository;
use async_graphql::{Context, Object};
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct CategoriesQueries;

#[Object]
impl CategoriesQueries {
    pub async fn categories(&self, ctx: &Context<'_>) -> Vec<CategoryGQL> {
        let repo = ctx
            .data::<Arc<CategoryRepository>>()
            .expect("CategoryRepository not in context");
        match repo.find_all().await {
            Ok(categories) => categories.into_iter().map(CategoryGQL::from).collect(),
            Err(_) => vec![],
        }
    }
}
