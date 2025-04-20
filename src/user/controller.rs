use actix_multipart::Multipart;
use actix_web::{HttpResponse, Responder, web};
use futures_util::StreamExt;
use uuid::Uuid;

use crate::{
    auth::middleware::AuthenticatedUser, contract::repository::VersionControl, storage::db::DbPool,
    user::repository::UserRepository,
};

pub async fn upload_user_avatar(
    vc: web::Data<VersionControl>,
    pool: web::Data<DbPool>,
    mut payload: Multipart,
    mut a_user: AuthenticatedUser,
) -> impl Responder {
    while let Some(field_result) = payload.next().await {
        let mut field = match field_result {
            Ok(f) => f,
            Err(e) => return HttpResponse::BadRequest().body(format!("Field error: {}", e)),
        };

        let file_name = Uuid::new_v4().to_string();
        let bucket_name = "user-data".to_string();
        let mut data = Vec::new();
        while let Some(chunk_result) = field.next().await {
            let chunk = match chunk_result {
                Ok(c) => c,
                Err(e) => return HttpResponse::BadRequest().body(format!("Chunk error: {}", e)),
            };
            data.extend_from_slice(&chunk);
        }

        if let Err(e) = vc
            .client
            .get_client()
            .put_object(&bucket_name, &file_name, data.into())
            .await
        {
            return HttpResponse::InternalServerError().body(format!("Upload failed: {}", e));
        }
        let mut conn = match pool.get() {
            Ok(c) => c,
            Err(_) => return HttpResponse::InternalServerError().body("Database connection error"),
        };
        let host = "http://localhost:9000".to_string();
        a_user.0.photo_url = Some(format!("{}/{}/{}", host, &bucket_name, &file_name));
        if let Err(e) = UserRepository::update(&mut conn, a_user.0) {
            return HttpResponse::InternalServerError().body(format!("Update failed: {}", e));
        }
        return HttpResponse::Ok().body("uploaded");
    }

    HttpResponse::BadRequest().body("no file found")
}
