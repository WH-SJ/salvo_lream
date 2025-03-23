use crate::errors::AppResult;
use crate::model::{UserRequest, UserResponse};
use crate::repository::user_repo::UserRepo;

pub struct UserServices<R: UserRepo> {
    repo: R
}

impl<R: UserRepo> UserServices<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
    
    pub async fn find_by_id(&self, id: i64) -> AppResult<UserResponse> {
        match self.repo.find_by_id(id).await {
            Ok(u) => Ok(UserResponse::from(u)),
            Err(e) => Err(e)
        }
    }
    
    pub async fn login(&self, user_request: UserRequest) -> AppResult<UserResponse> {
        
    }
}