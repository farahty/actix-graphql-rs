use crate::{
    db::repo::MongoRepository,
    error::Result,
    models::categories::{CategoryRepository, NewCategoryInput},
    schema::categories::CategoryGQL,
};

use async_graphql::{Context, Object};
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct CategoriesQueries;

#[Object]
impl CategoriesQueries {
    pub async fn categories(&self, ctx: &Context<'_>) -> Result<Vec<CategoryGQL>> {
        let repo = ctx.data::<Arc<CategoryRepository>>()?;
        let categories = repo.find_all().await?;
        Ok(categories.into_iter().map(CategoryGQL::from).collect())
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
    ) -> Result<CategoryGQL> {
        let repo = ctx.data::<Arc<CategoryRepository>>()?;

        Ok(repo.create(&input).await?.into())
    }
}
