use std::fs::File;
use std::io::{Write};
use base64::{Engine as _, engine::general_purpose};
use crate::DbPool;
use crate::models::*;
use crate::schema::cars_for_sale::dsl::*;
use diesel::prelude::*;
use actix_web::{get, post, patch, delete, web, HttpResponse, Error};


// TODO: fix InvalidPadding
fn save_img(vehicle: CarsForSale, output_dir: String) {
    /*
    let extension: String = vehicle.formato_imagem.unwrap().replace("image/", "").replace(";base64", "");
    let file_name: String = "generate_new_file_name".to_string();
    let file_path: String = format!("{}/{}.{}", output_dir, file_name, extension);
    println!("Exporting picture: {}", file_path);

    let encoded  = vehicle.foto.unwrap();
    let file_data = general_purpose::STANDARD_NO_PAD.decode(encoded).unwrap_or_else(|e| {
        println!("Error: {}", e);
        Vec::new()
    });
    if file_data.is_empty() { return }

    let mut file = File::create(file_path).unwrap();
    file.write_all(&file_data).unwrap_or_else(|e| {
        println!("Error: {}", e);
        return
    });
    file.flush().unwrap_or_else(|e| {
        println!("Error: {}", e);
        return
    });
    */
}

#[get("/cars")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let cars = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<Vec<CarsForSale>, diesel::result::Error> = cars_for_sale.load::<CarsForSale>(&mut conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(cars))
}

#[post("/cars")]
async fn create(pool: web::Data<DbPool>, payload: web::Json<CarsForSale>) -> Result<HttpResponse, Error> {
    // let image_payload: web::Json<ImagePayload> = payload;
    // let output_dir: String = "".to_string();
    // save_img(image_payload, output_dir);

    let car = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<usize, diesel::result::Error> = diesel::insert_into(cars_for_sale).values(payload.into_inner()).execute(&mut conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(car))
}

#[get("/cars/{car_id}")]
async fn select(pool: web::Data<DbPool>, car_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let car = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<Option<CarsForSale>, diesel::result::Error> = cars_for_sale.find(car_id.into_inner()).first(&mut conn).optional();
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(car))
}

#[patch("/cars/{car_id}")]
async fn update(pool: web::Data<DbPool>, car_id: web::Path<i32>, payload: web::Json<CarsForSale>) -> Result<HttpResponse, Error> {
    let car = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<usize, diesel::result::Error> = diesel::update(cars_for_sale.find(car_id.into_inner())).set(payload.into_inner()).execute(&mut conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(car))
}

#[delete("/cars/{car_id}")]
async fn delete(pool: web::Data<DbPool>, car_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let car = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<usize, diesel::result::Error> = diesel::delete(cars_for_sale.find(car_id.into_inner())).execute(&mut conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(car))
}
