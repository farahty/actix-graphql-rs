use axum::{response::Html, Extension};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_graphql::http::GraphiQLSource;

use crate::schema;

pub async fn endpoint(
    Extension(schema): Extension<schema::Schema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn ui() -> Html<String> {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
