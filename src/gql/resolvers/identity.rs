use async_graphql::Object;

#[derive(Default)]
pub struct IdentityQuery;

#[Object]
impl IdentityQuery {
    async fn identity(&self) -> String {
        String::from("identity")
    }
}

#[derive(Default)]
pub struct IdentityMutation;

#[Object]
impl IdentityMutation {
    async fn identity(&self) -> String {
        String::from("identity")
    }
}
