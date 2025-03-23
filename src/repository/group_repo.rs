use crate::errors::AppError;
use crate::model::{links, GroupActiveModel, GroupEntity, GroupModel, GroupRequest, UserModel};
use salvo::async_trait;
use sea_orm::{ActiveModelTrait, EntityTrait, DatabaseConnection, Set, ModelTrait};

#[async_trait]
pub trait GroupRepo: Send + Sync {
    async fn find_all(&self) -> Result<Vec<GroupModel>, AppError>;
    async fn find_by_id(&self, id: i64) -> Result<GroupModel, AppError>;
    async fn find_owned_users(&self, id: i64) -> Result<Vec<UserModel>, AppError>;
    async fn create(&self, group: GroupRequest) -> Result<(), AppError>;
}

pub struct PgGroupRepo<'a> {
    db: &'a DatabaseConnection
}

impl<'a> PgGroupRepo<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl GroupRepo for PgGroupRepo<'_> {
    async fn find_all(&self) -> Result<Vec<GroupModel>, AppError> {
        match GroupEntity::find().all(self.db).await {
            Ok(groups) => Ok(groups),
            Err(e) => Err(AppError::DbError(e))
        }
    }

    async fn find_by_id(&self, id: i64) -> Result<GroupModel, AppError> { 
        match GroupEntity::find_by_id(id).one(self.db).await {
            Ok(g) => {
                if let Some(group) = g {
                    return Ok(group)
                }
                Err(AppError::NotFound("The User Does Not Exist".to_string()))
            },
            Err(e) => Err(AppError::DbError(e)) 
        }
    }

    async fn find_owned_users(&self, id: i64) -> Result<Vec<UserModel>, AppError> {
        let group = self.find_by_id(id).await?;
        
        match group.find_linked(links::GroupToUser).all(self.db).await {
            Ok(users) => Ok(users),
            Err(e) => Err(AppError::DbError(e))
        }
    }

    async fn create(&self, group: GroupRequest) -> Result<(), AppError> {
        let group = GroupActiveModel {
            groupname: Set(group.name),
            ..Default::default()
        };

        if let Err(e) = group.insert(self.db).await {
            return Err(AppError::DbError(e))
        }
        
        Ok(())
    }
}