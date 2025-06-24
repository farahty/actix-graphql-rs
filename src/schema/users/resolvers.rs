use crate::{
    db::repo::MongoRepository,
    error::{Error, Result},
    models::users::{NewUserInput, UserRepository},
    schema::users::UserGQL,
    utils::password::{CheckPassword, HashPassword},
};

use async_graphql::{Context, Object, OneofObject};
use bson::doc;
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

#[derive(OneofObject)]
pub enum Identity {
    Email(String),
    Mobile(String),
}

#[Object]
impl UsersMutations {
    pub async fn create_user(&self, ctx: &Context<'_>, mut input: NewUserInput) -> Result<UserGQL> {
        let repo = UserRepository::new(ctx.data::<Database>()?);
        input.hash_password()?;
        Ok(repo.create(&input).await?.into())
    }

    pub async fn login(
        &self,
        ctx: &Context<'_>,
        identity: Identity,
        password: String,
    ) -> Result<Option<UserGQL>> {
        let repo = UserRepository::new(ctx.data::<Database>()?);
        let user = match identity {
            Identity::Email(email) => repo.find_one(doc! { "email": email }).await?,
            Identity::Mobile(mobile) => repo.find_one(doc! { "mobile": mobile }).await?,
        };

        match user {
            Some(user) => match user.check_password(password) {
                Ok(_) => Ok(Some(UserGQL::from(user))),
                Err(er) => Err(Error::new(format!("Invalid credentials ({}).", er.message))),
            },
            None => Err(Error::new("User not found")),
        }
    }
}
