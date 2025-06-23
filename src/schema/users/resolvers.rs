use crate::db::repo::MongoRepository;
use crate::error::Result;
use crate::models::users::{NewUserInput, UserRepository};
use crate::schema::users::UserGQL;

use async_graphql::{Context, Object};
use mongodb::Database;

#[derive(Default, Clone)]
pub struct UsersQueries;

#[Object]
impl UsersQueries {
    pub async fn users(&self, ctx: &Context<'_>) -> Result<Vec<UserGQL>> {
        let repo = UserRepository::new(ctx.data::<Database>()?);
        let users = repo.find_all().await?;

        Ok(users.into_iter().map(UserGQL::from).collect())
    }

    pub async fn user(&self, ctx: &Context<'_>, id: String) -> Result<Option<UserGQL>> {
        let repo = UserRepository::new(ctx.data::<Database>()?);
        let user = repo.find_by_id(&id).await?;

        Ok(user.map(UserGQL::from))
    }
}

#[derive(Default, Clone)]
pub struct UsersMutations;

#[Object]
impl UsersMutations {
    pub async fn create_user(&self, ctx: &Context<'_>, input: NewUserInput) -> Result<UserGQL> {
        let repo = UserRepository::new(ctx.data::<Database>()?);

        Ok(repo.create(&input).await?.into())
    }
}
