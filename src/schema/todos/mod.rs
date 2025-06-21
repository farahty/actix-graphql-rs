pub mod gql;
pub mod resolvers;
// Re-export for backward compatibility
pub use gql::TodoGQL;
pub use resolvers::TodosMutations;
pub use resolvers::TodosQueries;
