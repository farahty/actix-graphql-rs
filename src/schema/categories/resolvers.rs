use crate::db::repo::MongoRepository;
use crate::models::categories::{CategoryRepository, NewCategoryInput};
use crate::schema::categories::CategoryGQL;
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
