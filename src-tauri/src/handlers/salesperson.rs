use crate::DbPool;
use crate::models::*;
use crate::schema::salesperson::dsl::*;
use diesel::prelude::*;
use actix_web::{get, post, patch, delete, web, HttpResponse, Error};


#[get("/salespeople")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
     let salespeople = web::block(move || {
        let mut conn = pool.get().unwrap(); // TODO: fix unwrap
        let result: Result<Vec<Salesperson>, diesel::result::Error> = salesperson.load::<Salesperson>(&mut conn);
        return result;
     })
     .await?
     .map_err(actix_web::error::ErrorInternalServerError)?;

      Ok(HttpResponse::Ok().json(salespeople))
}

#[post("/salespeople")]
async fn create(pool: web::Data<DbPool>, payload: web::Json<Salesperson>) -> Result<HttpResponse, Error> {
   let salespersn = web::block(move || {
      let mut conn = pool.get().unwrap(); // TODO: fix unwrap
      let result: Result<usize, diesel::result::Error> = diesel::insert_into(salesperson).values(payload.into_inner()).execute(&mut conn);
      return result;
   })
   .await?
   .map_err(actix_web::error::ErrorInternalServerError)?;

   Ok(HttpResponse::Ok().json(salespersn))
}

//#[get("/salespeople/{sp_id}")]
//async fn select

// ...
// ...


#[patch("/salespeople/{sp_id}")]
async fn update(pool: web::Data<DbPool>, sp_id: web::Path<i32>, payload: web::Json<Salesperson>) -> Result<HttpResponse, Error> {
   let salespersn = web::block(move || {
      let mut conn = pool.get().unwrap(); // TODO: fix unwrap
      let result: Result<usize, diesel::result::Error> = diesel::update(salesperson.find(sp_id.into_inner())).set(payload.into_inner()).execute(&mut conn);
      return result;
   })
   .await?
   .map_err(actix_web::error::ErrorInternalServerError)?;

   Ok(HttpResponse::Ok().json(salespersn))
}

#[delete("/salespeople/{sp_id}")]
async fn delete(pool: web::Data<DbPool>, sp_id: web::Path<i32>) -> Result<HttpResponse, Error> {
   let salespersn = web::block(move || {
      let mut conn = pool.get().unwrap(); // TODO: fix unwrap
      let result: Result<usize, diesel::result::Error> = diesel::delete(salesperson.find(sp_id.into_inner())).execute(&mut conn);
      return result;
   })
   .await?
   .map_err(actix_web::error::ErrorInternalServerError)?;

   Ok(HttpResponse::Ok().json(salespersn))
}
