use farahty_macros::db_model;

use crate::gql_input_object;

#[db_model]
pub struct Todo {
    text: String,
    completed: bool,
    description: Option<String>,
}

gql_input_object!(NewTodoInput {
    text: String,
    completed: Option<bool>,
    description: Option<String>,
});

db_repository!(
    TodoRepository,
    Todo,
    NewTodoInput,
    "todos",
    to_entity {
        text: |input: &NewTodoInput| input.text.clone(),
        completed: |input: &NewTodoInput| input.completed.unwrap_or(false),
        description: |input: &NewTodoInput| input.description.clone(),
    }
);
