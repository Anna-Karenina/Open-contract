// src/minio_client.rs
use async_trait::async_trait;
use bytes::Bytes;
use minio_rsc::Minio;
use minio_rsc::error::Error as MinioError;
use minio_rsc::provider::StaticProvider;
use reqwest::Response;
use std::sync::Arc;

#[async_trait]
pub trait MinioOperations: Send + Sync {
    async fn bucket_exists(&self, bucket: &str) -> Result<bool, MinioError>;
    async fn make_bucket(&self, bucket: &str) -> Result<String, MinioError>;
    async fn put_object(&self, bucket: &str, object: &str, data: Bytes) -> Result<(), MinioError>;
    async fn get_object(&self, bucket: &str, object: &str) -> Result<Response, MinioError>;
    async fn remove_object(&self, bucket: &str, object: &str) -> Result<(), MinioError>;
}

pub struct MinioService {
    client: Arc<Minio>,
}

#[async_trait]
impl MinioOperations for MinioService {
    async fn bucket_exists(&self, bucket: &str) -> Result<bool, MinioError> {
        self.client.bucket_exists(bucket).await
    }

    async fn make_bucket(&self, bucket: &str) -> Result<String, MinioError> {
        self.client.make_bucket(bucket, false).await
    }

    async fn put_object(&self, bucket: &str, object: &str, data: Bytes) -> Result<(), MinioError> {
        self.client.put_object(bucket, object, data).await
    }

    async fn get_object(&self, bucket: &str, object: &str) -> Result<Response, MinioError> {
        self.client.get_object(bucket, object).await
    }

    async fn remove_object(&self, bucket: &str, object: &str) -> Result<(), MinioError> {
        self.client.remove_object(bucket, object).await
    }
}

impl MinioService {
    pub async fn new(
        endpoint: &str,
        access_key: &str,
        secret_key: &str,
        secure: bool,
    ) -> Result<Self, MinioError> {
        let provider = StaticProvider::new(access_key, secret_key, None);
        let client = Minio::builder()
            .endpoint(endpoint)
            .provider(provider)
            .secure(secure)
            .build()
            .expect("Cant connect to file storage");

        Ok(Self {
            client: Arc::new(client),
        })
    }

    pub async fn initialize_buckets(&self) -> Result<(), MinioError> {
        let buckets = ["proto-files", "proto-versions", "user-data"];

        for bucket in buckets.iter() {
            if !self.bucket_exists(bucket).await? {
                let _ = self.make_bucket(bucket).await?;
            }
        }

        Ok(())
    }

    pub fn get_client(&self) -> Arc<Minio> {
        Arc::clone(&self.client)
    }
}
