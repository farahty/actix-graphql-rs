use crate::db_model;
use crate::gql_input;

db_model!(Todo {
    text: String,
    completed: bool,
    description: Option<String>,
});

gql_input!(NewTodoInput {
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
