use crate::errors::AppResult;
use crate::model::{GroupResponse, UserResponse};
use crate::repository::group_repo::*;
use crate::DB_CONN;
use salvo::prelude::*;

#[endpoint]
async fn get_all_groups(res: &mut Response) -> AppResult<()>{
    let group_pg_repo = PgGroupRepo::new(DB_CONN.get().unwrap());
    let groups: Vec<GroupResponse> = group_pg_repo.find_all().await?.iter().map(|m| GroupResponse::from(m.clone())).collect();
    res.render(Json(groups));
    Ok(())
}

#[endpoint]
async fn get_group(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let group_pg_repo = PgGroupRepo::new(DB_CONN.get().unwrap());
    
    let id = req.param::<i64>("id").unwrap_or_default();
    
    let group: GroupResponse = group_pg_repo.find_by_id(id).await?.into();
    res.render(Json(group));
    Ok(())
}

#[endpoint]
async fn find_group_owned_users(req: &mut Request, res: &mut Response) -> AppResult<()> {
    let id = req.param::<i64>("id").unwrap_or_default();
    let group_pg_repo = PgGroupRepo::new(DB_CONN.get().unwrap());

    let groups: Vec<UserResponse> = group_pg_repo.find_owned_users(id).await?.iter().map(|m| UserResponse::from(m.clone())).collect();
    res.render(Json(groups));
    Ok(())
}

pub fn groups_route() -> Router {
    Router::with_path("group").push(
        Router::with_path("{id:num}").get(get_group)
            .push(Router::with_path("users").get(find_group_owned_users)))
        .push(Router::with_path("all").get(get_all_groups))
}