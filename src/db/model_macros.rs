#[macro_export]
macro_rules! db_model {
    // With field attributes
    ($name:ident { $($field:ident : $ftype:ty $(, #[$($attr:meta),*])?),* $(,)? }) => {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
            pub id: Option<mongodb::bson::oid::ObjectId>,
            pub created_at: Option<mongodb::bson::DateTime>,
            pub updated_at: Option<mongodb::bson::DateTime>,
            $(
                $(#[$($attr),*])?
                pub $field: $ftype,
            )*
        }
    };
}

#[macro_export]
macro_rules! gql_input {
    ($name:ident { $($field:ident : $ftype:ty),* $(,)? }) => {
        #[derive(async_graphql::InputObject, Clone)]
        pub struct $name {
            $(pub $field: $ftype,)*
        }
    };
}

#[macro_export]
macro_rules! db_repository {
    ($repo:ident, $db:ident, $coll:expr) => {
        #[derive(Clone)]
        pub struct $repo {
            collection: mongodb::Collection<mongodb::bson::Document>,
        }
        impl $repo {
            pub fn new(db: &mongodb::Database) -> Self {
                let collection = db.collection::<mongodb::bson::Document>($coll);
                Self { collection }
            }
        }
        impl $crate::db::repo::MongoRepository<$db, NewTodoInput> for $repo {
            fn collection(&self) -> &mongodb::Collection<mongodb::bson::Document> {
                &self.collection
            }
            // You may want to implement to_entity manually or extend the macro for custom logic
        }
    };
    (
        $repo:ident, $db:ident, $input:ident, $coll:expr,
        to_entity { $($field:ident : $extract:expr),* $(,)? }
    ) => {
        #[derive(Clone)]
        pub struct $repo {
            collection: mongodb::Collection<mongodb::bson::Document>,
        }
        impl $repo {
            pub fn new(db: &mongodb::Database) -> Self {
                let collection = db.collection::<mongodb::bson::Document>($coll);
                Self { collection }
            }
        }
        impl $crate::db::repo::MongoRepository<$db, $input> for $repo {
            fn collection(&self) -> &mongodb::Collection<mongodb::bson::Document> {
                &self.collection
            }
            fn to_entity(&self, input: &$input) -> $db {
                $db {
                    id: None,
                    $($field: $extract(input),)*
                    created_at: Some(mongodb::bson::DateTime::now()),
                    updated_at: Some(mongodb::bson::DateTime::now()),
                }
            }
        }
    };
}
