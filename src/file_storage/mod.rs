use std::env;

use minio_client::MinioService;

pub mod minio_client;

pub async fn establish_connection() -> MinioService {
    let minio_user = env::var("MINIO_ROOT_USER").expect("file storage user not provided");
    let minio_pass = env::var("MINIO_ROOT_PASSWORD").expect("File storage password not provided");
    let minio_host = env::var("MINIO_API_PORT").unwrap_or_else(|_| "9000".to_string());
    let minio_host = format!("localhost:{}", minio_host);

    let minio = MinioService::new(&minio_host, &minio_user, &minio_pass, false)
        .await
        .expect("Failure in file storage connect");

    minio
        .initialize_buckets()
        .await
        .expect("Failure in file  storage init");
    minio
}
