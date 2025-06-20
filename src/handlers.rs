use async_graphql::http::GraphiQLSource;
use axum::response::Html;

pub async fn ui() -> Html<String> {
    Html(GraphiQLSource::build().endpoint("/").finish())
}
