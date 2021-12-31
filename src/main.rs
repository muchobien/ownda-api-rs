use anyhow::Result;
use argon2::Argon2;
use async_graphql::{
    extensions::{ApolloTracing, Logger},
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    AddExtensionLayer, Router, Server,
};
use ownda::{
    domain::jwt::Claims,
    gql::{build_schema, GqlSchema},
};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use tower::ServiceBuilder;

async fn graphql_handler(
    Extension(conn): Extension<DatabaseConnection>,
    schema: Extension<GqlSchema>,
    req: GraphQLRequest,
    claims: Option<Claims>,
) -> GraphQLResponse {
    schema
        .execute(req.into_inner().data(conn).data(claims))
        .await
        .into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() -> Result<()> {
    let conn = Database::connect(env::var("DATABASE_URL")?).await?;
    let schema = build_schema()
        .data(Argon2::default())
        .extension(ApolloTracing)
        .extension(Logger)
        .finish();
    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(conn))
                .layer(AddExtensionLayer::new(schema)),
        );

    println!("Playground: http://localhost:8000");

    Server::bind(&"0.0.0.0:8000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
