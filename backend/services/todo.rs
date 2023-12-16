use actix_web::{delete, Error, get, HttpResponse, post, put, Result};

#[tsync::tsync]
#[derive(serde::Deserialize)]
pub struct PaginationParams {
    pub page: i64,
    pub page_size: i64,
}

#[get("")]
async fn index(
) -> HttpResponse {
    let result: Result<String, ()> = Ok("hi".to_string());

    if result.is_ok() {
        HttpResponse::Ok().json(result.unwrap())
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

#[get("/{id}")]
async fn read(
) -> HttpResponse {
    HttpResponse::NotFound().finish()
}

#[post("")]
async fn create(
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Created().finish())
}

#[put("/{id}")]
async fn update(
) -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[delete("/{id}")]
async fn destroy(
) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index)
        .service(read)
        .service(create)
        .service(update)
        .service(destroy);
}
