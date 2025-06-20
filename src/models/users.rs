use async_graphql::Enum;
use serde::Deserialize;
use serde::Serialize;

use crate::db_model;
use crate::gql_input_object;
use crate::gql_model;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Role {
    User,
    Admin,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum UserStatus {
    Active,
    Expired,
    Blocked,
    Suspended,
}

db_model!(User {
     name: Option<String>,
     email: Option<String>,
     password: Option<String>,
     mobile: Option<String>,
     token: Option<String>,
     otp_hash: Option<String>,
     role: Role,
     verified: bool,
     status: UserStatus
});

gql_model!(
    UserGQL,
    User {
        name: Option<String> = |value: &User| value.name.clone(),
        email: Option<String> = |value: &User| value.email.clone(),
        mobile: Option<String> = |value: &User| value.mobile.clone(),
        role: Role = |value: &User| value.role,
        verified: bool = |value: &User| value.verified,
        status: UserStatus = |value: &User| value.status,
        token: Option<String> = |value: &User| value.token.clone(),
        otp_hash: Option<String> = |value: &User| value.otp_hash.clone(),
    }
);

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
