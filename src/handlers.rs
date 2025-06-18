use actix_web::{get, post, web, HttpResponse, Result};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::schema;

#[post("/graphql")]
pub async fn endpoint(
    schema: web::Data<schema::Schema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

#[get("/")]
pub async fn ui() -> Result<HttpResponse> {
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
          <head>
            <meta charset="utf-8" />
            <title>GraphiQL</title>
            <link href="https://unpkg.com/graphiql@2.0.9/graphiql.min.css" rel="stylesheet" />
          </head>
          <body style="margin: 0; overflow: hidden;">
            <div id="graphiql" style="height: 100vh;"></div>
            <script
              crossorigin
              src="https://unpkg.com/react@17/umd/react.production.min.js"
            ></script>
            <script
              crossorigin
              src="https://unpkg.com/react-dom@17/umd/react-dom.production.min.js"
            ></script>
            <script
              src="https://unpkg.com/graphiql@2.0.9/graphiql.min.js"
            ></script>
            <script>
              const fetcher = GraphiQL.createFetcher({{ url: '/graphql' }});
              ReactDOM.render(
                React.createElement(GraphiQL, {{ fetcher }}),
                document.getElementById('graphiql'),
              );
            </script>
          </body>
        </html>
        "#
    );

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html))
}
