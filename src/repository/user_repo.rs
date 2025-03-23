use crate::errors::AppError;
use crate::model::{
    links,
    GroupModel,
    UserActiveModel,
    UserEntity,
    UserModel,
    UserRequest
};
use salvo::async_trait;
use sea_orm::{
    ActiveModelTrait,
    DatabaseConnection,
    EntityTrait,
    ModelTrait,
    Set
};

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn find_all(&self) -> Result<Vec<UserModel>, AppError>;
    async fn find_by_id(&self, id: i64) -> Result<UserModel, AppError>;

    async fn find_owned_group(&self, id: i64) -> Result<Vec<GroupModel>, AppError>;
    async fn create(&self, user: UserRequest) -> Result<(), AppError>;
}

pub struct PgUserRepo<'a> {
    db: &'a DatabaseConnection
}

impl<'a> PgUserRepo<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepo for PgUserRepo<'_> {
    async fn find_all(&self) -> Result<Vec<UserModel>, AppError> {
       match UserEntity::find().all(self.db).await {
           Ok(users) => Ok(users),
           Err(e) => Err(AppError::DbError(e))
       }
    }

    async fn find_by_id(&self, id: i64) -> Result<UserModel, AppError> {
        match UserEntity::find_by_id(id).one(self.db).await {
            Ok(u) => {
                if let Some(res) = u {
                    return Ok(res)
                }
                Err(AppError::NotFound("The User Does Not Exist".to_string()))
            }
            Err(e) => Err(AppError::DbError(e))
        }
    }

    async fn find_owned_group(&self, id: i64) -> Result<Vec<GroupModel>, AppError> {
        let user = self.find_by_id(id).await?;
        
        match user.find_linked(links::UserToGroup).all(self.db).await {
            Ok(groups) => Ok(groups), 
            Err(e) => Err(AppError::DbError(e))
        }
    }

    async fn create(&self, user: UserRequest) -> Result<(), AppError> {
        let user = UserActiveModel {
            username: Set(user.name),
            password: Set(user.passwd),
            ..Default::default()
        };

        if let Err(e) = user.insert(self.db).await { 
            return Err(AppError::DbError(e))
        }
        Ok(())
    }
}