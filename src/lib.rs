use std::sync::OnceLock;
use salvo::{handler, FlowCtrl, Listener, Response, Router, Server, Service};
use salvo::catcher::Catcher;
use salvo::conn::TcpListener;
use salvo::http::StatusCode;
use salvo::oapi::OpenApi;
use salvo::prelude::{SwaggerUi, Text};
use sea_orm::{Database, DatabaseConnection};
use crate::routes::groups::groups_route;
use crate::routes::users::users_route;

pub mod model;
pub mod repository;
pub mod errors;
pub mod services;
pub mod middleware;
pub mod routes;

pub static DB_CONN: OnceLock<DatabaseConnection> = OnceLock::new();

pub async fn set_db_conn() {
    let db = Database::connect("postgres://root:WRXwsp@20051231@10.0.110.1:20001/test?currentSchema=test").await.unwrap();
    DB_CONN.set(db).expect("Failed to initialize database connection");
}

#[handler]
async fn root(res: &mut Response) {
    res.render(Text::Html("<h1>This is a Salvo + Sea_orm + pg_db App Demo<h1>"));
}

#[handler]
async fn handle404(res: &mut Response, ctrl: &mut FlowCtrl) {
    if StatusCode::NOT_FOUND == res.status_code.unwrap_or(StatusCode::NOT_FOUND) {
        res.render("Custom 404 Error Page");
        ctrl.skip_rest();
    }
}

#[handler]
async fn error500(res: &mut Response) {
    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
}

fn root_route() -> Router {
    Router::new().get(root)
}

pub async fn app() {
    tracing_subscriber::fmt().init();
    set_db_conn().await;

    let router = root_route()
        .push(users_route())
        .push(groups_route())
        .push(Router::with_path("error").get(error500));
    
    let doc = OpenApi::new("Api", "0.1").merge_router(&router);

    let router = router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("api"));

    let service = Service::new(router).catcher(Catcher::default().hoop(handle404));

    let acceptor = TcpListener::new("0.0.0.0:5110").bind().await;
    Server::new(acceptor).serve(service).await;
}