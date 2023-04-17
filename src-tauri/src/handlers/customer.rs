use crate::DbPool;
use crate::models::*;
use crate::schema::customer::dsl::*;
use diesel::prelude::*;
use actix_web::{get, post, patch, delete, web, HttpResponse, Error};


#[get("/customers")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let customers = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<Vec<Customer>, diesel::result::Error> = customer.load::<Customer>(&mut conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(customers))
}

#[post("/customers")]
async fn create(pool: web::Data<DbPool>, payload: web::Json<Customer>) -> Result<HttpResponse, Error> {
    let customr = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<usize, diesel::result::Error> = diesel::insert_into(customer).values(payload.into_inner()).execute(&mut conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Created().json(customr))
}

#[get("/customers/{customr_id}")]
async fn select(pool: web::Data<DbPool>, customr_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let customr = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<Option<Customer>, diesel::result::Error> = customer.find(customr_id.into_inner()).first(&mut conn).optional();
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(customr))
}

#[patch("/customers/{customr_id}")]
async fn update(pool: web::Data<DbPool>, customr_id: web::Path<i32>, payload: web::Json<Customer>) -> Result<HttpResponse, Error> {
    let customr = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<usize, diesel::result::Error> = diesel::update(customer.find(customr_id.into_inner())).set(payload.into_inner()).execute(&mut conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(customr))
}

#[delete("/customers/{customr_id}")]
async fn delete(pool: web::Data<DbPool>, customr_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let customr = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<usize, diesel::result::Error> = diesel::delete(customer.find(customr_id.into_inner())).execute(&mut conn);
        return result;
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(customr))
}
