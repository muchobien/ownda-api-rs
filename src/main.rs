use std::net::SocketAddr;

use anyhow::Result;
use argon2::Argon2;
use async_graphql::extensions;
use axum::{AddExtensionLayer, Server};
use ownda::{db, gql::build_schema, routes, settings::SETTINGS};
use sea_orm::Database;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let conn = Database::connect(SETTINGS.database.get_connect_options()).await?;

    db::migrate(&conn).await;

    let schema = build_schema()
        .data(Argon2::default())
        .extension(extensions::ApolloTracing)
        .extension(extensions::Logger)
        .finish();

    let app = routes::gql().layer(
        ServiceBuilder::new()
            .layer(AddExtensionLayer::new(conn))
            .layer(AddExtensionLayer::new(schema)),
    );

    let addr = SETTINGS.application.get_address()?;
    print_init_messages(&addr)?;

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

fn print_init_messages(addr: &SocketAddr) -> Result<()> {
    log::info!("Server running on {}", addr);
    log::info!(
        "GraphQL link: http://{}{}",
        addr,
        SETTINGS.application.graphql.path
    );

    Ok(())
}
