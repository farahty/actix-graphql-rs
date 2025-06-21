use crate::models::todos::Todo;

#[derive(async_graphql::SimpleObject, Clone, Debug)]
pub struct TodoGQL {
    pub id: Option<String>,
    pub text: String,
    pub completed: bool,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<Todo> for TodoGQL {
    fn from(value: Todo) -> Self {
        TodoGQL {
            id: value.id.as_ref().map(|oid| oid.to_hex()),
            text: value.text.clone(),
            completed: value.completed,
            description: value.description.clone(),
            created_at: value.created_at.map(|dt| dt.to_string()),
            updated_at: value.updated_at.map(|dt| dt.to_string()),
        }
    }
}
