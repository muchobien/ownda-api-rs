use async_graphql::Enum;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "Identity")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: Provider,
    pub hash: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum, Enum, Copy, Eq)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "provider_enum")]
pub enum Provider {
    #[sea_orm(string_value = "APPLE")]
    Apple,
    #[sea_orm(string_value = "GOOGLE")]
    Google,
    #[sea_orm(string_value = "LOCAL")]
    Local,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
