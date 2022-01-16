use crate::entity::{
    prelude::{Account, Category, Identity, Transaction, User},
    sea_orm_active_enums::{ProviderEnum, TransactionTypeEnum},
};
use sea_orm::{ActiveEnum, ConnectionTrait, DatabaseConnection, EntityTrait, Schema};

async fn create_table<E: EntityTrait>(conn: &DatabaseConnection, entity: E) {
    let builder = conn.get_database_backend();
    let schema = Schema::new(builder);
    let stmt = builder.build(schema.create_table_from_entity(entity).if_not_exists());

    match conn.execute(stmt).await {
        Ok(_) => println!("Migrated {}", entity.table_name()),
        Err(e) => println!("Error: {}", e),
    }
}

async fn create_enum<A: ActiveEnum>(conn: &DatabaseConnection) {
    let builder = conn.get_database_backend();
    let schema = Schema::new(builder);
    let stmt = builder.build(&schema.create_enum_from_active_enum::<A>());

    match conn.execute(stmt).await {
        Ok(_) => println!("Migrated {}", A::name()),
        Err(e) => println!("Error: {}", e),
    }
}

pub async fn migrate(conn: &DatabaseConnection) {
    create_table(conn, User).await;
    create_table(conn, Account).await;
    create_table(conn, Category).await;
    create_enum::<TransactionTypeEnum>(conn).await;
    create_table(conn, Transaction).await;
    create_enum::<ProviderEnum>(conn).await;
    create_table(conn, Identity).await;
}
