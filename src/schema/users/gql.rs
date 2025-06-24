use crate::models::users::{Role, User, UserStatus};

// Define UserGQL with field exclusions and renames
// This approach is limited but doesn't require procedural macros
#[derive(async_graphql::SimpleObject, Clone, Debug)]
pub struct UserGQL {
    pub id: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>, // Renamed from mobile
    pub role: Role,
    pub verified: bool,
    pub status: UserStatus,
    pub token: Option<String>,
    pub otp_hash: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<User> for UserGQL {
    fn from(value: User) -> Self {
        UserGQL {
            id: value.id.as_ref().map(|oid| oid.to_hex()),
            name: value.name.clone(),
            email: value.email.clone(),
            phone_number: value.mobile.clone(), // Renamed field
            role: value.role,
            verified: value.verified,
            status: value.status,
            token: value.token.clone(),
            otp_hash: value.otp_hash.clone(),
            created_at: value.created_at.map(|dt| dt.to_string()),
            updated_at: value.updated_at.map(|dt| dt.to_string()),
        }
    }
}
