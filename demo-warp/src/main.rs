mod security;
mod todo_rest;

use std::{convert::Infallible, sync::Arc};

use crate::todo_rest::todos_filter;
use warp::Filter;

const WEB_FOLDER: &str = "web-folder/";

#[tokio::main]
async fn main() {
    let db_pool = Arc::new(DbPool {});
    // APIs
    let hi = warp::path("hi").and(warp::get()).map(|| "Hello from hi");
    let apis = hi.or(todos_filter(db_pool.clone()));

    // Static content
    let content = warp::fs::dir(WEB_FOLDER);
    let root = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", WEB_FOLDER)));
    let static_site = content.or(root);

    let routes = apis.or(static_site);

    println!("start web-server");
    warp::serve(routes).run(([0, 0, 0, 0], 8888)).await;
}

pub struct DbPool {}

pub fn with_db_pool(
    db_pool: Arc<DbPool>,
) -> impl Filter<Extract = (Arc<DbPool>,), Error = Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}
