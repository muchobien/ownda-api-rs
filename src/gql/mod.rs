pub mod context;
mod guard;
mod resolvers;

use self::resolvers::{
    account::{AccountMutation, AccountQuery},
    category::{CategoryMutation, CategoryQuery},
    transaction::{TransactionMutation, TransactionQuery},
    user::{UserMutation, UserQuery},
};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

#[derive(MergedObject, Default)]
pub struct Query(AccountQuery, CategoryQuery, TransactionQuery, UserQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(
    AccountMutation,
    CategoryMutation,
    TransactionMutation,
    UserMutation,
);

pub type GqlSchema = Schema<Query, Mutation, EmptySubscription>;

pub type GqlSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> GqlSchemaBuilder {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}
