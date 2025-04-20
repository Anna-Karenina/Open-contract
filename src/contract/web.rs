use crate::storage::db::DbPool;
use crate::user::repository::UserRepository;
use crate::{auth::middleware::AuthenticatedUser, project::repository::ProjectRepository};

use actix_web::{HttpResponse, Responder, web};
use askama::Template;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

use super::{
    comment_model::Comment,
    model::{Contract, DiffLine, VersionMetadata},
    repository::{CommentsRepository, ContractRepository, VersionControl},
};

#[derive(Template)]
#[template(path = "pages/contract/contract.html")]

pub struct IndexContractTemplate {
    pub username: String,
    pub user_photo_url: String,
    pub active_tab: ContractTabParams,
    pub project_id: i32,
}

pub struct CommentMessage {
    pub published_at: String,
    pub author_self: bool,
    pub author_name: String,
    pub text: String,
}

#[derive(Template)]
#[template(path = "pages/contract/components/service-card-comments.html")]
pub struct ServiceCardCommentsComponent {
    messages: Vec<CommentMessage>,
    contract_id: i32,
}

#[derive(Template)]
#[template(path = "pages/contract/components/service-card.html")]
pub struct ServiceCardTemplate<'a> {
    pub service: &'a Contract,
}

#[derive(Template)]
#[template(path = "pages/contract/components/drawer.html")]
pub struct ContractDrawerTemplate {}

#[derive(Template)]
#[template(path = "pages/contract/components/drawer.html")]

pub struct ContractAsideTemplate {}

#[derive(Template)]
#[template(path = "pages/contract/components/content-services.html")]
pub struct ContentService<'a> {
    pub services_by_tags: &'a HashMap<String, Vec<String>>,
}

#[derive(Template)]
#[template(path = "pages/contract/components/content-editor.html")]
pub struct ContentEditor {
    pub filename: String,
    pub proto_content: String,
}

#[derive(Template)]
#[template(path = "pages/contract/components/content-import.html")]
pub struct ContentImport {}

#[derive(Template)]
#[template(path = "pages/contract/components/drawer.html")]

// #[template(path = "contract/version_history.html")]
pub struct VersionHistoryTemplate {
    pub filename: String,
    pub versions: Vec<VersionMetadata>,
}

#[derive(Template)]
#[template(path = "pages/contract/components/drawer.html")]

// #[template(path = "contract/diff_view.html")]
pub struct DiffViewTemplate {
    pub filename: String,
    pub version1: String,
    pub version2: String,
    pub diff: Vec<DiffLine>,
}

pub async fn index(
    project_id: web::Path<i32>,
    query: web::Query<ContractTabs>,
    a_user: AuthenticatedUser,
) -> impl Responder {
    let project_id = project_id.into_inner();
    let active_tab = query
        .tab
        .as_ref()
        .cloned()
        .unwrap_or(ContractTabParams::Services);

    let template = IndexContractTemplate {
        user_photo_url: a_user.0.photo_url.unwrap_or_else(|| String::from("")),
        username: a_user.0.name,
        active_tab,
        project_id,
    };

    match template.render() {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn render_drawer(path: web::Path<i32>) -> impl Responder {
    let _project_id = path.into_inner();

    let template = ContractDrawerTemplate {};

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

pub async fn service(pool: web::Data<DbPool>, project_id: web::Path<i32>) -> impl Responder {
    let result = match web::block(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        let project_id = project_id.into_inner();
        ContractRepository::get_contracts_by_project(&mut conn, project_id)
    })
    .await
    {
        Ok(Ok(contracts)) => {
            let mut tags: HashMap<String, Vec<Contract>> = HashMap::new();
            contracts.into_iter().for_each(|contract| {
                let t = contract.tag.clone().unwrap_or_else(|| "".to_string());
                tags.entry(t).or_insert_with(Vec::new).push(contract);
            });
            tags
        }
        Ok(Err(_)) | Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to fetch contracts");
        }
    };

    let mut services_by_tags: HashMap<String, Vec<String>> = HashMap::new();

    for (tag, contracts) in result {
        let rendered_services = contracts
            .iter()
            .map(|contract| {
                ServiceCardTemplate { service: contract }
                    .render()
                    .map_err(|e| {
                        HttpResponse::InternalServerError()
                            .body(format!("Failed to render service template: {}", e))
                    })
            })
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        services_by_tags.insert(tag, rendered_services);
    }

    let template = ContentService {
        services_by_tags: &services_by_tags,
    };

    match template.render() {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn editor(
    pool: web::Data<DbPool>,
    project_id: web::Path<i32>,
    vc: web::Data<VersionControl>,
) -> impl Responder {
    let project_id = project_id.into_inner();
    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        ProjectRepository::find_by_id(&mut conn, project_id)
    })
    .await;

    let project = match result {
        Ok(project_option) => match project_option {
            Ok(Some(project)) => project,
            Ok(None) => return HttpResponse::NotFound().body("Project not found"),
            Err(_) => {
                return HttpResponse::InternalServerError().body("Failed to fetch project details");
            }
        },
        Err(_) => return HttpResponse::InternalServerError().body("Failed to fetch project"),
    };

    let file_name = match project.proto_file {
        Some(file) => file,
        None => return HttpResponse::InternalServerError().body("Failed to get file namee"),
    };

    let proto_content = match vc.read_file_content(&file_name).await {
        Ok(content) => content,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to get file content: {}", e));
        }
    };

    let template = ContentEditor {
        filename: file_name,
        proto_content,
    };

    match template.render() {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn comments(
    pool: web::Data<DbPool>,
    path_params: web::Path<(i32, i32)>,
    a_user: AuthenticatedUser,
) -> impl Responder {
    let (_, service_id) = path_params.into_inner();

    let comments = match fetch_comments(pool.clone(), service_id).await {
        Ok(comments) => comments,
        Err(err) => return err,
    };

    let messages = match enrich_comments_with_author(pool, comments, a_user).await {
        Ok(messages) => messages,
        Err(err) => return err,
    };

    let template = ServiceCardCommentsComponent {
        messages,
        contract_id: service_id,
    };
    match template.render() {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn fetch_comments(
    pool: web::Data<DbPool>,
    service_id: i32,
) -> Result<Vec<Comment>, HttpResponse> {
    web::block(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        CommentsRepository::find_comments_by_service(&mut conn, service_id)
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError().body("Failed to fetch comments"))?
    .map_err(|_| HttpResponse::InternalServerError().body("Failed to fetch comments"))
}

async fn enrich_comments_with_author(
    pool: web::Data<DbPool>,
    comments: Vec<Comment>,
    a_user: AuthenticatedUser,
) -> Result<Vec<CommentMessage>, HttpResponse> {
    let mut conn = pool.get().map_err(|_| {
        HttpResponse::InternalServerError().body("Failed to get database connection")
    })?;

    Ok(comments
        .into_iter()
        .map(|comment| {
            let author_name = UserRepository::find_by_id(&mut conn, comment.author)
                .map(|user| user.name)
                .unwrap_or_else(|_| "Unknown".to_string());

            CommentMessage {
                published_at: comment.created_at.format("%H:%M %d.%m.%y").to_string(),
                author_self: comment.author == a_user.0.id,
                author_name,
                text: comment.comment,
            }
        })
        .collect())
}

pub async fn import() -> impl Responder {
    let template = ContentImport {};
    match template.render() {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn render_aside() -> impl Responder {
    let template = ContractAsideTemplate {};

    match template.render() {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
pub struct ContractTabs {
    pub tab: Option<ContractTabParams>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContractTabParams {
    Services,
    Editor,
    Import,
}

impl<'de> Deserialize<'de> for ContractTabParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.to_lowercase().as_str() {
            "editor" => ContractTabParams::Editor,
            "import" => ContractTabParams::Import,
            _ => ContractTabParams::Services,
        })
    }
}

impl std::fmt::Display for ContractTabParams {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Services => write!(f, "services"),
            Self::Editor => write!(f, "editor"),
            Self::Import => write!(f, "import"),
        }
    }
}
impl ContractTabParams {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Services => "services",
            Self::Editor => "editor",
            Self::Import => "import",
        }
    }

    pub fn matches(&self, value: &str) -> bool {
        self.as_str() == value
    }
}
