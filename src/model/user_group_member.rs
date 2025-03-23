use sea_orm::prelude::*;
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::group::Entity",
        from = "Column::GroupId",
        to = "super::group::Column::Id"
    )]
    Group
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(schema_name = "test", table_name = "user_group_member")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i64,
    user_id: i64,
    group_id: i64,
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
    is_delete: bool,
}

impl ActiveModelBehavior for ActiveModel {}