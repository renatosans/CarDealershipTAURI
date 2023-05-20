use std::fs::File;
use std::io::{Write};
use std::path::PathBuf;
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
    pub image_format: String,
    pub image_data: String
}

// TODO: fix InvalidPadding
fn save_img(payload: VehiclePayload, output_dir: String) -> String {
    let extension: String = payload.image_format.replace("image/", "").replace(";base64", "");
    let file_name: String = format!("generate_new_file_name_{}", payload.image_data.len());
    let file_path: String = format!("{}/{}.{}", output_dir, file_name.clone(), extension.clone());
    println!("Saving img on File System: {}", file_path);

    let encoded  = payload.image_data;
    let file_data = general_purpose::STANDARD.decode(encoded).unwrap_or_else(|e| {
        println!("Error: {}", e);
        Vec::new()
    });
    if file_data.is_empty() {
        return "".to_string();
    }

    let mut file = File::create(file_path).unwrap();
    file.write_all(&file_data).unwrap_or_else(|e| {
        println!("Error: {}", e);
        return
    });
    file.flush().unwrap_or_else(|e| {
        println!("Error: {}", e);
        return
    });

    return format!("/img/cars/{}.{}", file_name, extension);
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

#[derive(Deserialize)]
struct PriceRange {
    min: Option<u32>,
    max: Option<u32>,
}

// TODO: filter cars by price, example   http://localhost:8080/api/cars_by_price?min=999&max=99900
#[get("/cars_by_price")]
async fn cars_by_price(pool: web::Data<DbPool>, params: web::Query<PriceRange>) -> Result<HttpResponse, Error> {
    let min = params.min.unwrap();
    let max = params.max.unwrap();

    let filters: Vec<String> = vec![min.to_string(), max.to_string()];
    Ok(HttpResponse::Ok().json(filters))
}

#[derive(Deserialize)]
struct CarFilter {
    yearFrom: Option<u32>,
    yearTo: Option<u32>,
}

// TODO: filter cars by price, mileage and year
#[get("/cars_filtered")]
async fn cars_filtered(pool: web::Data<DbPool>, params: web::Query<CarFilter>) -> Result<HttpResponse, Error> {
    let yearFrom = params.yearFrom.unwrap();
    let yearTo = params.yearTo.unwrap();

    let filters: Vec<String> = vec![yearFrom.to_string(), yearTo.to_string()];
    Ok(HttpResponse::Ok().json(filters))
}

#[post("/cars")]
async fn create(pool: web::Data<DbPool>, payload: web::Json<VehiclePayload>) -> Result<HttpResponse, Error> {
    let payload: VehiclePayload = payload.into_inner();

    // TODO: verificar se essa gravaçao no File System é muito custosa
    let mut path = PathBuf::new();
    path.push(std::env::current_dir().unwrap());
    path.pop(); // sobe um diretório, verificar em produção como fica
    path.push("public");

    let output_dir = format!("{}{}", path.display(), "/img/cars");
    std::fs::create_dir_all(output_dir.clone()).unwrap();

    let saved_file: String = save_img(payload.clone(), output_dir);
    let new_car = CarsForSale{
        id: payload.id,
        brand: payload.brand,
        model: payload.model,
        year: payload.year,
        img: Some(saved_file),     // get the image path from the image file being created
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
