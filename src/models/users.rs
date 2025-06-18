use crate::db_model;
use crate::gql_model;
use crate::gql_input_object;

db_model!(User {
    username: String,
    email: String,
});

gql_model!(
    UserGQL,
    User {
        username: String = |value: &User| value.username.clone(),
        email: String = |value: &User| value.email.clone(),
    }
);

gql_input_object!(NewUserInput {
    username: String,
    email: String,
});

db_repository!(UserRepository, User, NewUserInput, "users", to_entity {
    username: |input: &NewUserInput| input.username.clone(),
    email: |input: &NewUserInput| input.email.clone(),
});
