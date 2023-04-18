use std::fs::File;
use std::io::{Write};
use base64::{Engine as _, engine::general_purpose};
use crate::DbPool;
use crate::models::*;
use crate::schema::cars_for_sale::dsl::*;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use actix_web::{get, post, patch, delete, web, HttpResponse, Error};


#[derive(Serialize, Deserialize, Clone)]
pub struct VehiclePayload {
    pub id: i32,
    pub brand: String,
    pub model: String,
    pub year: i32,
    pub img: Option<String>,
    pub color: Option<String>,
    pub mileage: Option<i32>,
    pub category: Option<String>,
    pub price: f64,
    pub imageFormat: String,
    pub imageData: String
}

// TODO: fix InvalidPadding
fn save_img(payload: VehiclePayload, output_dir: String) {
    let extension: String = payload.imageFormat.replace("image/", "").replace(";base64", "");
    let file_name: String = "generate_new_file_name".to_string();
    let file_path: String = format!("{}/{}.{}", output_dir, file_name, extension);
    println!("Saving img on File System: {}", file_path);

    let encoded  = payload.imageData;
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
async fn create(pool: web::Data<DbPool>, payload: web::Json<VehiclePayload>) -> Result<HttpResponse, Error> {
    let payload: VehiclePayload = payload.into_inner();

    // TODO: verificar se essa gravaçao no File System é muito custosa
    let output_dir = format!("{}{}", std::env::current_dir().unwrap().display(), "/img");
    std::fs::create_dir_all(output_dir.clone()).unwrap();

    save_img(payload.clone(), output_dir);
    let new_car = CarsForSale{
        id: payload.id,
        brand: payload.brand,
        model: payload.model,
        year: payload.year,
        img: Some("".to_string()),     // get the image path from the image file being created
        color: payload.color,
        mileage: payload.mileage,
        category: payload.category,
        price: payload.price
    };

    let car = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<usize, diesel::result::Error> = diesel::insert_into(cars_for_sale).values(new_car).execute(&mut conn);
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
