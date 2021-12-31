mod guard;
mod resolvers;
pub mod context;

use self::resolvers::{
    identity::{IdentityMutation, IdentityQuery},
    user::{UserMutation, UserQuery},
};
use async_graphql::{EmptySubscription, MergedObject, Schema, SchemaBuilder};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, IdentityQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation, IdentityMutation);

pub type GqlSchema = Schema<Query, Mutation, EmptySubscription>;

pub type GqlSchemaBuilder = SchemaBuilder<Query, Mutation, EmptySubscription>;

pub fn build_schema() -> GqlSchemaBuilder {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
}
