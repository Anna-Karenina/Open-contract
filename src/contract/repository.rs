use crate::file_storage::minio_client::MinioService;
use crate::schema::{comments, contracts};

use bytes::Bytes;
use chrono::Utc;
// use diesel::{
//     ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, query_dsl::methods::FilterDsl,
// };

use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::prelude::*;

use std::sync::Arc;
use uuid::Uuid;

use super::comment_model::{Comment, NewComment};
use super::model::CrateContract;
use super::model::{Contract, VersionMetadata};

#[derive(Clone)]

pub struct VersionControl {
    pub client: Arc<MinioService>, //refactor!
}

impl VersionControl {
    pub fn new(client: Arc<MinioService>) -> Self {
        Self { client }
    }

    pub async fn save_version(
        &self,
        filename: &str,
        content: &str,
        user_id: &str,
    ) -> Result<VersionMetadata, minio_rsc::error::Error> {
        let version_id = format!("{}-{}", Utc::now().timestamp(), Uuid::new_v4());
        let version_key = format!("{}/{}.proto", filename, version_id);
        let metadata_key = format!("{}/_versions.json", filename);

        // Get existing metadata or create new
        let mut versions = self.get_versions(filename).await.unwrap_or_default();

        // Add new version
        let new_version = VersionMetadata {
            id: version_id.clone(),
            timestamp: Utc::now().to_rfc3339(),
            author: user_id.to_string(),
            size: content.len(),
        };
        versions.insert(0, new_version.clone());

        // Save version file

        self.client
            .get_client()
            .put_object(
                "proto-versions",
                &version_key,
                Bytes::from(content.as_bytes().to_vec()),
            )
            .await?;

        // Update metadata
        let metadata_json = serde_json::to_string(&versions)
            .map_err(|e| minio_rsc::error::Error::ValueError(e.to_string()))?;
        self.client
            .get_client()
            .put_object(
                "proto-versions",
                &metadata_key,
                Bytes::from(metadata_json.as_bytes().to_vec()),
            )
            .await?;

        // Update current file
        self.client
            .get_client()
            .put_object(
                "proto-files",
                filename,
                Bytes::from(content.as_bytes().to_vec()),
            )
            .await?;

        Ok(new_version)
    }

    pub async fn get_versions(
        &self,
        filename: &str,
    ) -> Result<Vec<VersionMetadata>, minio_rsc::error::Error> {
        let metadata_key = format!("{}/_versions.json", filename);

        match self
            .client
            .get_client()
            .get_object("proto-versions", &metadata_key)
            .await
        {
            Ok(response) => {
                let data = response.bytes().await?;
                Ok(serde_json::from_slice(&data)
                    .map_err(|e| minio_rsc::error::Error::ValueError(e.to_string()))?)
            }
            Err(e) if e.to_string().contains("NoSuchKey") => Ok(Vec::new()),
            Err(e) => Err(e),
        }
    }

    pub async fn get_version_content(
        &self,
        filename: &str,
        version_id: &str,
    ) -> Result<String, minio_rsc::error::Error> {
        let version_key = format!("{}/{}.proto", filename, version_id);
        let response = self
            .client
            .get_client()
            .get_object("proto-versions", &version_key)
            .await?;
        let data = response.bytes().await?;
        Ok(String::from_utf8(data.to_vec())
            .map_err(|e| minio_rsc::error::Error::ValueError(e.to_string()))?)
    }

    pub async fn read_file_content(
        &self,
        file_name: &str,
    ) -> Result<String, minio_rsc::error::Error> {
        let response = self
            .client
            .get_client()
            .get_object("proto-files", file_name)
            .await?;
        let data = response.bytes().await?;
        Ok(String::from_utf8(data.to_vec())
            .map_err(|e| minio_rsc::error::Error::ValueError(e.to_string()))?)
    }
}

pub struct ContractRepository;

impl ContractRepository {
    pub fn get_contracts_by_project(
        conn: &mut PgConnection,
        p_ip: i32,
    ) -> Result<Vec<Contract>, diesel::result::Error> {
        use crate::schema::contracts::dsl::*;

        contracts.filter(project_id.eq(p_ip)).load::<Contract>(conn)
    }

    pub fn create_contract(
        conn: &mut PgConnection,
        contract: CrateContract,
    ) -> Result<Contract, diesel::result::Error> {
        Err(diesel::result::Error::NotFound)
        // diesel::insert_into(contracts::table)
        //     .values(contract)
        //     .get_result(conn)
    }
}

pub struct CommentsRepository;

impl CommentsRepository {
    pub fn create_comment(
        conn: &mut PgConnection,
        comment: NewComment,
    ) -> Result<Comment, diesel::result::Error> {
        diesel::insert_into(comments::table)
            .values(comment)
            .get_result(conn)
    }

    pub fn find_comments_by_service(
        conn: &mut PgConnection,
        s_id: i32,
    ) -> Result<Vec<Comment>, diesel::result::Error> {
        use crate::schema::comments::dsl::*;

        comments
            .filter(contract_id.eq(s_id))
            .order_by(created_at.asc())
            .load::<Comment>(conn)
    }
}
