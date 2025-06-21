use async_graphql::Enum;
use farahty_macros::db_model;
use serde::Deserialize;
use serde::Serialize;

use crate::gql_input_object;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Role {
    User,
    Admin,
}

impl Default for Role {
    fn default() -> Self {
        Self::User
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum UserStatus {
    Active,
    Expired,
    Blocked,
    Suspended,
}

impl Default for UserStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[db_model]
pub struct User {
    #[serde(default)]
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    mobile: Option<String>,
    token: Option<String>,
    otp_hash: Option<String>,
    #[serde(default)]
    role: Role,
    #[serde(default)]
    verified: bool,
    #[serde(default)]
    status: UserStatus,
}

gql_input_object!(NewUserInput {
    name: Option<String>,
    email: Option<String>,
    password: String,
    mobile: Option<String>,
});

db_repository!(
    UserRepository,
    User,
    NewUserInput,
    "users",
    to_entity {
        name: |input: &NewUserInput| input.name.clone(),
        email: |input: &NewUserInput| input.email.clone(),
        password: |input: &NewUserInput| Some(input.password.clone()),
        mobile: |input: &NewUserInput| input.mobile.clone(),
        role: |_: &NewUserInput| Role::User,
        verified: |_: &NewUserInput| false,
        status: |_: &NewUserInput| UserStatus::Active,
        token: |_: &NewUserInput| None,
        otp_hash: |_: &NewUserInput| None,
    }
);
