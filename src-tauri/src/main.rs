// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod schema;
mod models;
mod backend;
mod handlers;

use std::thread;
use backend::{serve};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};


pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

// O  TAURI é utilizado para aplicações semelhantes ao Electron porem não foi utilizado aqui
// com este propósito, ele está sendo utilizado para prototipagem no estilo monolito, algo
// semelhande ao que o NEXTjs, Nuxt ou REMIX fornecem
//
// Este projeto não usa o Custom Protocol fornecido pelo TAURI, pois não se enquadra nas práticas
// mais comuns de mercado  REST API, GraphQL, tRPC, etc...
// Ao inves disso é utilizada uma API padrão de mercado (REST API)

// The purpose of this project is to provide a developer experience similar to NEXTjs, Nuxt
// or REMIX, using a frontend and backend packed toghether
// The protocol provided by TAURI was not used in this project, instead a REST API is used

// Frontend at http://127.0.0.1:1420/
// Backend at http://127.0.0.1:8080/api/cars


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // Abre uma Thread para o backend
    thread::spawn(move || {
        serve().expect("Unable to start web server");
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
