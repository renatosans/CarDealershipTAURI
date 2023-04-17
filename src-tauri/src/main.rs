// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod schema;
mod models;
mod handlers;


use std::thread;
use dotenv::dotenv;
use handlers::customer;
use handlers::salesperson;
use handlers::cars_for_sale;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use actix_cors::Cors;
use actix_web::{web, middleware, App, HttpServer};

pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    thread::spawn(move || {
        serve().expect("Unable to start web server");
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[actix_web::main]
async fn serve() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(|| async { "Actix REST API" }))
            .service(
                web::scope("/api")
                    .service(cars_for_sale::index)
                    .service(cars_for_sale::select)
                    .service(cars_for_sale::create)
                    .service(cars_for_sale::update)
                    .service(cars_for_sale::delete)
                    .service(customer::index)
                    .service(customer::select)
                    .service(customer::create)
                    .service(customer::update)
                    .service(customer::delete)
                    .service(salesperson::index)
                    // .service(salesperson::select)
                    // .service(salesperson::create)
                    // .service(salesperson::update)
                    .service(salesperson::delete)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
