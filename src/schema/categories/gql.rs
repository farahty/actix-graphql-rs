use crate::models::categories::Category;

#[derive(async_graphql::SimpleObject, Clone, Debug)]
pub struct CategoryGQL {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<Category> for CategoryGQL {
    fn from(value: Category) -> Self {
        CategoryGQL {
            id: value.id.as_ref().map(|oid| oid.to_hex()),
            name: value.name.clone(),
            description: value.description.clone(),
            created_at: value.created_at.map(|dt| dt.to_string()),
            updated_at: value.updated_at.map(|dt| dt.to_string()),
        }
    }
}
