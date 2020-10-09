use actix_rt;
use actix_files as fs;
use actix_web::{web, App, HttpServer, middleware};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use std::io;

mod config;
mod db;
mod handlers;
pub mod models;
mod websocket;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres", NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("Starting server at http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(fs::Files::new("/", "static/").index_file("index.html"))
            .route("/ws/", web::get().to(websocket::index))
            .route("/todos{_:/?}", web::get().to(handlers::get_lists))
            .route("/todos{_:/?}", web::post().to(handlers::create_todo_list))
            .route("/todos/{list_id}{_:/?}", web::get().to(handlers::get_todos))
            .route("/todos/{list_id}{_:/?}",
                   web::post().to(handlers::create_todo),
            )
            .route("/todos/{list_id}/{item_id}{_:/?}",
                   web::get().to(handlers::get_todo),
            )
            .route("/todos/{list_id}/{item_id}{_:/?}",
                   web::put().to(handlers::check_todo),
            )
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
