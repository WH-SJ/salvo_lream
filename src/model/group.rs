use sea_orm::prelude::*;
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(schema_name = "test", table_name = "user_groups")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i64,
    groupname: String,
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
    is_delete: bool,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_group_member::Relation::User.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_group_member::Relation::Group.def())
    }
}

#[derive(Serialize, Deserialize)]
pub struct GroupRequest {
    pub id: i64,
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct GroupResponse {
    pub id: i64,
    pub name: String,
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
}

impl From<Model> for GroupResponse {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            name: m.groupname,
            create_time: m.create_time,
            update_time: m.update_time
        }
    }
}
