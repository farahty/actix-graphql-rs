use crate::db_model;
use crate::gql_model;
use crate::gql_input_object;

db_model!(Todo {
    text: String,
    completed: bool,
    description: Option<String>,
});

gql_model!(
    TodoGQL,
    Todo {
        text: String = |value: &Todo| value.text.clone(),
        completed: bool = |value: &Todo| value.completed,
        description: Option<String> = |value: &Todo| value.description.clone(),
    }
);

gql_input_object!(NewTodoInput {
    text: String,
    completed: Option<bool>,
    description: Option<String>,
});

db_repository!(TodoRepository, Todo, NewTodoInput, "todos", to_entity {
    text: |input: &NewTodoInput| input.text.clone(),
    completed: |input: &NewTodoInput| input.completed.unwrap_or(false),
    description: |input: &NewTodoInput| input.description.clone(),
});
