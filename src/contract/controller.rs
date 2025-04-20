use actix_web::{HttpResponse, Responder, web};
use askama::Template;

use crate::{
    auth::middleware::AuthenticatedUser, contract::web::DiffViewTemplate, storage::db::DbPool,
};

use super::{
    comment_model::NewComment,
    model::{DiffLine, SaveFileRequest},
    repository::{CommentsRepository, VersionControl},
    web::VersionHistoryTemplate,
};

pub async fn save_version(
    vc: web::Data<VersionControl>,
    filename: web::Path<String>,
    content: web::Json<SaveFileRequest>,
    a_user: AuthenticatedUser,
) -> impl Responder {
    let content_str = content.into_inner().content;
    let user_id = a_user.0.id.to_string();
    match vc
        .save_version(&filename.into_inner(), &content_str, &user_id)
        .await
    {
        Ok(version) => HttpResponse::Ok().json(version),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_versions(
    vc: web::Data<VersionControl>,
    filename: web::Path<String>,
) -> impl Responder {
    match vc.get_versions(&filename).await {
        Ok(versions) => {
            let template = VersionHistoryTemplate {
                filename: filename.to_string(),
                versions,
            };
            HttpResponse::Ok().body(template.render().unwrap())
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn get_version_content(
    vc: web::Data<VersionControl>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (filename, version_id) = path.into_inner();
    match vc.get_version_content(&filename, &version_id).await {
        Ok(content) => HttpResponse::Ok().body(content),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn diff_versions(
    vc: web::Data<VersionControl>,
    path: web::Path<(String, String, String)>,
) -> impl Responder {
    let (filename, version1, version2) = path.into_inner();
    let content1 = vc.get_version_content(&filename, &version1).await.unwrap();
    let content2 = vc.get_version_content(&filename, &version2).await.unwrap();

    dbg!(&content1);
    dbg!(&content2);
    let diff = simple_diff(&content1, &content2);

    let template = DiffViewTemplate {
        filename,
        version1,
        version2,
        diff,
    };

    HttpResponse::Ok().body(template.render().unwrap())
}

fn simple_diff(content1: &str, content2: &str) -> Vec<DiffLine> {
    let lines1: Vec<&str> = content1.lines().collect();
    let lines2: Vec<&str> = content2.lines().collect();
    let max_lines = lines1.len().max(lines2.len());
    let mut diff = Vec::new();

    for i in 0..max_lines {
        let line1 = lines1.get(i).unwrap_or(&"");
        let line2 = lines2.get(i).unwrap_or(&"");

        if line1 != line2 {
            diff.push(DiffLine {
                line_number: i + 1,
                old_line: line1.to_string(),
                new_line: line2.to_string(),
            });
        }
    }

    diff
}

pub async fn create_comments(
    comment: web::Json<NewComment>,
    pool: web::Data<DbPool>,
    a_user: AuthenticatedUser,
) -> impl Responder {
    let mut comment = comment.into_inner();
    comment.author = Some(a_user.0.id);
    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        CommentsRepository::create_comment(&mut conn, comment)
    })
    .await;
    match result {
        Ok(Ok(project)) => HttpResponse::Created().json(project),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        Ok(Err(_)) => HttpResponse::InternalServerError().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
