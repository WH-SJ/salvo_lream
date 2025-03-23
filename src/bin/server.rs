// static DB_CONN: OnceLock<DatabaseConnection> = OnceLock::new();
// 
// #[handler]
// async fn root(res: &mut Response) {
//     res.render(Text::Html("<h1>This is a Salvo + Sea_orm + pg_db App Demo<h1>"));
// }
// 
// #[handler]
// async fn hello() -> &'static str {
//     "holle world"
// }
// 
// #[handler]
// async fn get_all_user(res: &mut Response) {
//     let user_pg_repo = PgUserRepo::new(DB_CONN.get().unwrap());
//     let user: Vec<UserResponse> = user_pg_repo.find_all().await.unwrap().iter().map(|m| UserResponse::from(m.clone())).collect();
//     res.render(Json(user));
// }
// 
// #[handler]
// async fn find_user_owned_groups(req: &mut Request, res: &mut Response) -> AppResult<()> {
//     let id = req.param::<i32>("id").unwrap_or_default();
//     let user_pg_repo = PgUserRepo::new(DB_CONN.get().unwrap());
//     
//     
//     let user: Vec<GroupResponse> = user_pg_repo.find_owned_group(id).await?.iter().map(|m| GroupResponse::from(m.clone())).collect();
//     res.render(Json(user));
//     Ok(())
// }
// 
// #[handler]
// async fn get_user(req: &mut Request, res: &mut Response) -> AppResult<()>{
//     let user_pg_repo = PgUserRepo::new(DB_CONN.get().unwrap());
// 
//     let id =  req.param::<i32>("id").unwrap_or_default();
//     
//     let user: UserResponse = user_pg_repo.find_by_id(id).await?.into();
//     res.render(Json(user));
//     Ok(())
// }
// 
// 
// #[handler]
// async fn get_all_groups(res: &mut Response) -> AppResult<()>{
//     let group_pg_repo = PgGroupRepo::new(DB_CONN.get().unwrap());
//     let groups: Vec<GroupResponse> = group_pg_repo.find_all().await?.iter().map(|m| GroupResponse::from(m.clone())).collect();
//     res.render(Json(groups));
//     Ok(())
// }
// 
// #[handler]
// async fn get_group(req: &mut Request, res: &mut Response) -> AppResult<()> {
//     let group_pg_repo = PgGroupRepo::new(DB_CONN.get().unwrap());
//     let id = req.param::<i32>("id").unwrap_or_default();
//     
//     let group: GroupResponse = group_pg_repo.find_by_id(id).await?.into();
//     res.render(Json(group));
//     Ok(())
// }

// #[handler]
// async fn find_group_owned_users(req: &mut Request, res: &mut Response) {
//     let id = req.param::<i32>("id").unwrap_or_default();
//     let group_pg_repo = PgGroupRepo::new(DB_CONN.get().unwrap());
//     let groups: Vec<UserResponse> = group_pg_repo.find_owned_users(id).await.unwrap().iter().map(|m| UserResponse::from(m.clone())).collect();
//     res.render(Json(groups));
// }

// #[handler]
// async fn handle404(res: &mut Response, ctrl: &mut FlowCtrl) {
//     if StatusCode::NOT_FOUND == res.status_code.unwrap_or(StatusCode::NOT_FOUND) {
//         res.render("Custom 404 Error Page");
//         ctrl.skip_rest();
//     }
// }
// 
// #[handler]
// async fn error500(res: &mut Response) {
//     res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
// }

use salvo_lream::app;

#[tokio::main]
async fn main() {
    app().await
    
    // tracing_subscriber::fmt().init();
    // 
    // let db = Database::connect("postgres://root:WRXwsp@20051231@10.0.110.1:20001/test?currentSchema=test").await.unwrap();
    // 
    // DB_CONN.set(db).expect("Failed to initialize database connection");
    // 
    // let router = Router::new().get(root)
    //     .push(
    //         Router::with_path("hello").get(hello)
    //     ).push(
    //         Router::with_path("user").push(
    //             Router::with_path("{id:num}").get(get_user).push(
    //                 Router::with_path("groups").get(find_user_owned_groups)
    //             )
    //         ).push(
    //             Router::with_path("all").get(get_all_user)
    //         )
    //     ).push(
    //         Router::with_path("group").push(
    //             Router::with_path("{id:num}").get(get_group)
    //             //     .push(
    //             //     Router::with_path("users").get(find_group_owned_users)
    //             // )
    //         ).push(
    //             Router::with_path("all").get(get_all_groups)
    //         )
    //     ).push(
    //         Router::with_path("error").get(error500)
    //     );
    // 
    // let service = Service::new(router).catcher(Catcher::default().hoop(handle404));
    // 
    // let acceptor = TcpListener::new("0.0.0.0:5110").bind().await;
    // Server::new(acceptor).serve(service).await;
}