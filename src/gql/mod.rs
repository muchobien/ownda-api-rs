pub mod context;
mod guard;
mod resolvers;

use self::resolvers::{
    account::{AccountMutation, AccountQuery},
    transaction::TransactionQuery,
    user::{UserMutation, UserQuery},
};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, AccountQuery, TransactionQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, AccountMutation);

pub type GqlSchema = Schema<Query, Mutation, EmptySubscription>;

pub type GqlSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> GqlSchemaBuilder {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}
