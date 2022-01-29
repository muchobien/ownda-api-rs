use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{self, IntoResponse},
    routing::post,
    Router,
};
use sea_orm::DatabaseConnection;

use crate::{domain::jwt::Claims, gql::GqlSchema, settings::SETTINGS};

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
    response::Html(playground_source(GraphQLPlaygroundConfig::new(
        &SETTINGS.application.graphql.path,
    )))
}

async fn forbidden_response() -> impl IntoResponse {
    StatusCode::FORBIDDEN
}

pub fn gql() -> Router {
    let gql_config = &SETTINGS.application.graphql;
    let schema_router = Router::new();

    match gql_config.playground {
        true => schema_router.route(
            &gql_config.path,
            post(graphql_handler).get(graphql_playground),
        ),
        false => schema_router.route(
            &gql_config.path,
            post(graphql_handler).get(forbidden_response),
        ),
    }
}
