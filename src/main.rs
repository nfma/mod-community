use std::error::Error;

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::Extension, response::{self, IntoResponse}, Router, routing::get, Server};

use async_graphql::{EmptyMutation, EmptySubscription, http::GraphiQLSource, Object, Schema};

struct Query;

#[Object]
impl Query {
    async fn howdy(&self) -> &'static str {
        "partner"
    }
}

async fn graphql_handler(
    schema: Extension<Schema<Query, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    println!("GraphiQL: http://localhost:8000");

    let addr = &"0.0.0.0:8000".parse()?;
    Server::bind(addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
