use crate::db_model;
use crate::gql_model;
use crate::gql_input_object;

db_model!(Category {
    name: String,
    description: Option<String>,
});

gql_model!(CategoryGQL, Category {
    name: String = |value: &Category| value.name.clone(),
    description: Option<String> = |value: &Category| value.description.clone(),
});

gql_input_object!(NewCategoryInput {
    name: String,
    description: Option<String>,
});

db_repository!(CategoryRepository, Category, NewCategoryInput, "categories", to_entity {
    name: |input: &NewCategoryInput| input.name.clone(),
    description: |input: &NewCategoryInput| input.description.clone(),
});
