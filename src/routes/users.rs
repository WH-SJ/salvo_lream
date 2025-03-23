use crate::errors::{AppError, AppResult};
use crate::model::{GroupResponse, UserResponse};
use crate::repository::user_repo::{PgUserRepo, UserRepo};
use crate::DB_CONN;
use salvo::prelude::Json;
use salvo::{handler, Request, Response, Router};
use salvo::oapi::endpoint;

#[endpoint]
async fn get_all_user(res: &mut Response) -> AppResult<()> {
    let user_pg_repo = PgUserRepo::new(DB_CONN.get().unwrap());
    let user: Vec<UserResponse> = user_pg_repo.find_all().await?.iter().map(|m| UserResponse::from(m.clone())).collect();
    res.render(Json(user));
    Ok(())
}

#[endpoint]
async fn find_user_owned_groups(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let id = req.param::<i64>("id").unwrap_or_default();
    let user_pg_repo = PgUserRepo::new(DB_CONN.get().unwrap());
    
    let user: Vec<GroupResponse> = user_pg_repo.find_owned_group(id).await?.iter().map(|m| GroupResponse::from(m.clone())).collect();
    res.render(Json(user));
    Ok(())
}

#[endpoint]
async fn get_user(req: &mut Request, res: &mut Response) -> AppResult<()>{
    let user_pg_repo = PgUserRepo::new(DB_CONN.get().unwrap());

    let id =  req.param::<i64>("id").unwrap_or_default();

    let user: UserResponse = user_pg_repo.find_by_id(id).await?.into();
    res.render(Json(user));
    Ok(())
}

#[handler]
async fn get_user_v2(req: &mut Request, res: &mut Response) -> AppResult<()>{
    let user_pg_repo = PgUserRepo::new(DB_CONN.get().unwrap());

    let id= match req.query::<i64>("id") {
        None => {
            return Err(AppError::InsufficientParameters("User Id Not Found".to_string()))
        }
        Some(i) => i
    };

    let user: UserResponse = user_pg_repo.find_by_id(id).await?.into();
    res.render(Json(user));
    Ok(())
}

pub fn users_route() -> Router {
    Router::with_path("user")
        .push(
            Router::with_path("{id:num}").get(get_user)
                .push(Router::with_path("groups").get(find_user_owned_groups)))
        .push(Router::with_path("all").get(get_all_user))
        .push(Router::with_path("v2").get(get_user_v2))
}