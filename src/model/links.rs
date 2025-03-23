use sea_orm::{LinkDef, Linked, RelationTrait};

pub struct UserToGroup;

impl Linked for UserToGroup {
    type FromEntity = super::user::Entity;
    type ToEntity = super::group::Entity;

    fn link(&self) -> Vec<LinkDef> {
        vec![
            super::user_group_member::Relation::User.def().rev(),
            super::user_group_member::Relation::Group.def()
        ]
    }
}

pub struct GroupToUser;

impl Linked for GroupToUser {
    type FromEntity = super::group::Entity;
    type ToEntity = super::user::Entity;

    fn link(&self) -> Vec<LinkDef> {
        vec![
            super::user_group_member::Relation::Group.def().rev(),
            super::user_group_member::Relation::User.def()
        ]
    }
}