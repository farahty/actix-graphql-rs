use crate::db::repo::MongoRepository;
use crate::error::Result;
use crate::models::users::{NewUserInput, UserRepository};
use crate::schema::users::UserGQL;

use async_graphql::{Context, Object};
use std::sync::Arc;

#[derive(Default, Clone)]
pub struct UsersQueries;

#[Object]
impl UsersQueries {
    pub async fn users(&self, ctx: &Context<'_>) -> Result<Vec<UserGQL>> {
        let repo = ctx.data::<Arc<UserRepository>>()?;
        let users = repo.find_all().await?;
        Ok(users.into_iter().map(UserGQL::from).collect())
    }

    pub async fn user(&self, ctx: &Context<'_>, id: String) -> Result<Option<UserGQL>> {
        let repo = ctx.data::<Arc<UserRepository>>()?;

        let user = repo.find_by_id(&id).await?;
        Ok(user.map(UserGQL::from))
    }
}

#[derive(Default, Clone)]
pub struct UsersMutations;

#[Object]
impl UsersMutations {
    pub async fn create_user(&self, ctx: &Context<'_>, input: NewUserInput) -> Option<UserGQL> {
        let repo = ctx
            .data::<Arc<UserRepository>>()
            .expect("UserRepository not in context");
        match repo.create(&input).await {
            Ok(user) => Some(UserGQL::from(user)),
            Err(_) => None,
        }
    }
}
