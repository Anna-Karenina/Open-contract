use actix_web::HttpResponse;
use serde::Serialize;

pub type WebResponse<E> = Result<HttpResponse, E>;

pub fn ok_response<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(data)
}
