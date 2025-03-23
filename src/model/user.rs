use chrono::NaiveDateTime;
use sea_orm::{
    prelude::*,
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(schema_name = "test", table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    id: i64,
    username: String,
    #[serde(skip_deserializing)]
    password: String,
    phone: String,
    email: String,
    create_time: NaiveDateTime,
    update_time: NaiveDateTime,
    is_delete: bool,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::group::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_group_member::Relation::Group.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::user_group_member::Relation::User.def())
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub id: i64,
    pub name: String,
    pub passwd: String,
    pub phone: String,
    pub email: String
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i64,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

impl From<Model> for UserResponse {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            name: m.username,
            phone: m.phone,
            email: m.email,
            create_time: m.create_time,
            update_time: m.update_time
        }
    }
}