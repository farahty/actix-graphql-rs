use crate::db::{
    repo::MongoRepository,
    users::{NewUserInput, UserGQL, UserRepository},
};
use async_graphql::{Context, Object};
use std::sync::Arc;

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
