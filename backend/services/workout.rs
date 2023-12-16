use actix_web::{HttpResponse, get};

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

pub fn endpoints(scope: actix_web::Scope) -> actix_web::Scope {
    return scope
        .service(index);
}
