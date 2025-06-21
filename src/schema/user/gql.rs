use crate::models::users::{Role, UserStatus};

#[derive(async_graphql::SimpleObject, Clone, Debug)]
pub struct UserGQL {
    pub id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,

    #[graphql(default = "Role::User")]
    pub role: Role,
    pub verified: bool,
    pub status: UserStatus,
    pub token: Option<String>,
    pub otp_hash: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<crate::models::users::User> for UserGQL {
    fn from(value: crate::models::users::User) -> Self {
        UserGQL {
            id: value.id.as_ref().map(|oid| oid.to_hex()),
            name: value.name.clone(),
            email: value.email.clone(),
            mobile: value.mobile.clone(),
            role: value.role,
            verified: value.verified,
            status: value.status,
            token: value.token.clone(),
            otp_hash: value.otp_hash.clone(),
            created_at: value.created_at.as_ref().map(|dt| dt.to_string()),
            updated_at: value.updated_at.as_ref().map(|dt| dt.to_string()),
        }
    }
}
