use crate::db_model;
use crate::gql_input;

db_model!(Category {
    name: String,
    description: Option<String>,
});

gql_input!(NewCategoryInput {
    name: String,
    description: Option<String>,
});

db_repository!(
    CategoryRepository,
    Category,
    NewCategoryInput,
    "categories",
    to_entity {
        name: |input: &NewCategoryInput| input.name.clone(),
        description: |input: &NewCategoryInput| input.description.clone(),
    }
);
