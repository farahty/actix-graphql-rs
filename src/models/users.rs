use async_graphql::Enum;
use serde::Deserialize;
use serde::Serialize;

use crate::gql_input;
use crate::utils::otp::CheckOTP;
use crate::utils::password::CheckPassword;
use crate::utils::password::HashPassword;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum Role {
    #[default]
    User,
    Admin,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize, Default)]
pub enum UserStatus {
    #[default]
    Active,
    Expired,
    Blocked,
    Suspended,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    #[serde(default)]
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub mobile: Option<String>,
    pub token: Option<String>,
    pub otp_hash: Option<String>,

    #[serde(default)]
    pub role: Role,

    #[serde(default)]
    pub verified: bool,

    #[serde(default)]
    pub status: UserStatus,

    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<mongodb::bson::oid::ObjectId>,
    pub created_at: Option<mongodb::bson::DateTime>,
    pub updated_at: Option<mongodb::bson::DateTime>,
}

impl CheckOTP for User {
    fn get_hashed_otp(&self) -> Option<&str> {
        self.otp_hash.as_deref()
    }
}

impl CheckPassword for User {
    fn get_hashed_password(&self) -> Option<String> {
        self.password.clone()
    }
}

gql_input!(NewUserInput {
    name: Option<String>,
    email: Option<String>,
    password: String,
    mobile: Option<String>,
});

impl HashPassword for NewUserInput {
    fn get_password(&self) -> Option<String> {
        Some(self.password.clone())
    }

    fn set_password(&mut self, password: String) {
        self.password = password;
    }
}

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
