use crate::db::repo::MongoRepository;
use crate::models::users::{UserGQL, UserRepository};
use async_graphql::{Context, Object, Result};
use mongodb::bson::oid::ObjectId;
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
        let oid = ObjectId::parse_str(&id)?;

        let user = repo.find_by_id(oid).await?;
        Ok(user.map(UserGQL::from))
    }
}
