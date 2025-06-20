#[macro_export]
macro_rules! db_model {
    ($name:ident { $($field:ident : $ftype:ty),* $(,)? }) => {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub struct $name {
            #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
            pub id: Option<mongodb::bson::oid::ObjectId>,
            pub created_at: Option<mongodb::bson::DateTime>,
            pub updated_at: Option<mongodb::bson::DateTime>,
            $(
                pub $field: $ftype,
            )*
        }
    };
}

#[macro_export]
macro_rules! gql_model {
    ($gql:ident, $db:ident { $($field:ident : $ftype:ty = $extract:expr),* $(,)? }) => {
        #[derive(async_graphql::SimpleObject, Clone)]
        pub struct $gql {
            pub id: Option<String>,
            $(pub $field: $ftype,)*
            pub created_at: Option<String>,
            pub updated_at: Option<String>,
        }
        impl From<$db> for $gql {
            fn from(value: $db) -> Self {
                $gql {
                    id: value.id.as_ref().map(|oid| oid.to_hex()),
                    $( $field: ($extract)(&value), )*
                    created_at: value.created_at.as_ref().map(|dt| dt.to_string()),
                    updated_at: value.updated_at.as_ref().map(|dt| dt.to_string()),
                }
            }
        }
    };
}

// Helper macro to infer the type of the field from the extraction expression
#[macro_export]
macro_rules! gql_model_field_type {
    ($db:ident, $field:ident, $extract:expr) => {
        <_ as ::core::ops::Deref>::Target
    };
}

#[macro_export]
macro_rules! gql_input_object {
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
                    created_at: None,
                    updated_at: None,
                }
            }
        }
    };
}
