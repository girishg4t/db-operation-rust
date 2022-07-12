use crate::error_handler::CustomError;
use crate::model::{Employee, Employees, User, Users};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/users/{id}")]
async fn find_user(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let user = Users::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create_user(user: web::Json<User>) -> Result<HttpResponse, CustomError> {
    let user = Users::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/employees/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let employee = Employees::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[post("/employees")]
async fn create(employee: web::Json<Employee>) -> Result<HttpResponse, CustomError> {
    let employee = Employees::create(employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[put("/employees/{id}")]
async fn update(
    id: web::Path<i32>,
    employee: web::Json<Employee>,
) -> Result<HttpResponse, CustomError> {
    let employee = Employees::update(id.into_inner(), employee.into_inner())?;
    Ok(HttpResponse::Ok().json(employee))
}

#[delete("/employees/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_employee = Employees::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_employee })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);

    config.service(find_user);
    config.service(create_user);
}
