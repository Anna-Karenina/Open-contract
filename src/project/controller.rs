use crate::{
    auth::middleware::AuthenticatedUser,
    contract::{
        model::{CrateContract, ServiceRequest},
        repository::{ContractRepository, VersionControl},
    },
    proto_parser,
    storage::db::DbPool,
    utils,
};
use actix_web::{HttpResponse, Responder, web};
use diesel::result::Error;
use dot_proto_parser::ProtoParser;

use super::{
    model::{NewProject, UpdateProject},
    repository::ProjectRepository,
};

pub async fn create_project(
    pool: web::Data<DbPool>,
    project: web::Json<NewProject>,
    vc: web::Data<VersionControl>,
) -> impl Responder {
    let mut new_project = project.into_inner();
    new_project.proto_file = Some(format!(
        "{}.proto",
        utils::strings_utils::replace_non_alphanumeric(new_project.name.as_str())
    ));
    let file_name = new_project.proto_file.clone().expect("file name is epmty");

    if let Err(err) = vc.save_version(file_name.as_str(), "", "123").await {
        return HttpResponse::InternalServerError().body(format!("Version control error: {}", err));
    }

    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        ProjectRepository::create(&mut conn, new_project)
    })
    .await;

    match result {
        Ok(Ok(project)) => HttpResponse::Created().json(project),
        Ok(Err(Error::NotFound)) => HttpResponse::NotFound().finish(),
        Ok(Err(_)) => HttpResponse::InternalServerError().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_all_projects(pool: web::Data<DbPool>, _u: AuthenticatedUser) -> impl Responder {
    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        ProjectRepository::find_all(&mut conn)
    })
    .await;

    match result {
        Ok(Ok(projects)) => HttpResponse::Ok().json(projects),
        Ok(Err(_)) => HttpResponse::InternalServerError().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_project(pool: web::Data<DbPool>, project_id: web::Path<i32>) -> impl Responder {
    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        ProjectRepository::find_by_id(&mut conn, *project_id)
    })
    .await;

    match result {
        Ok(Ok(Some(project))) => HttpResponse::Ok().json(project),
        Ok(Ok(None)) => HttpResponse::NotFound().finish(),
        Ok(Err(_)) => HttpResponse::InternalServerError().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_project(
    pool: web::Data<DbPool>,
    project_id: web::Path<i32>,
    project: web::Json<UpdateProject>,
) -> impl Responder {
    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        ProjectRepository::update(&mut conn, *project_id, project.into_inner())
    })
    .await;

    match result {
        Ok(Ok(project)) => HttpResponse::Ok().json(project),
        Ok(Err(Error::NotFound)) => HttpResponse::NotFound().finish(),
        Ok(Err(_)) => HttpResponse::InternalServerError().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete_project(pool: web::Data<DbPool>, project_id: web::Path<i32>) -> impl Responder {
    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        ProjectRepository::delete(&mut conn, *project_id)
    })
    .await;

    match result {
        Ok(Ok(_)) => HttpResponse::NoContent().finish(),
        Ok(Err(Error::NotFound)) => HttpResponse::NotFound().finish(),
        Ok(Err(_)) => HttpResponse::InternalServerError().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn crate_contract(
    pool: web::Data<DbPool>,
    service: web::Json<ServiceRequest>,
    project_id: web::Path<i32>,
    a_user: AuthenticatedUser,
    vc: web::Data<VersionControl>,
) -> impl Responder {
    let service = service.into_inner();
    let project_id = project_id.into_inner();
    let user_id = a_user.0.id;

    let body = match serde_json::to_string(&service.body) {
        Ok(it) => Some(it),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let parsed_response = match serde_json::to_string(&service.response) {
        Ok(it) => Some(it),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let (parsed_path, parsed_query_params) =
        proto_parser::utils::parse_path_template(&service.path);

    let grpc_method = format!(
        "{}{}",
        utils::strings_utils::capitalize(&service.method),
        parsed_path
    );

    let q = format!(
        "{{\n{},\n{}\n}}",
        service.path_params,
        parsed_query_params.join(": string,\n")
    );

    let payload: CrateContract = CrateContract {
        project_id,
        author_id: a_user.0.id,
        grpc_method,
        tag: Some(service.tag.clone()),
        errors_response: Some("errors".to_string()),
        path: Some(service.path.clone()),
        query: Some(q),
        body,
        response: parsed_response,
        http_method: Some(service.method.clone()),
        description: Some(service.description.clone()),
    };

    let cloned_pool = pool.clone();

    let project = web::block(move || {
        let mut conn = match cloned_pool.get() {
            Ok(conn) => conn,
            Err(_) => return None,
        };
        match ProjectRepository::find_by_id(&mut conn, project_id) {
            Ok(project) => project,
            Err(_) => return None,
        }
    })
    .await;

    let proto_file_name = match project {
        Ok(option_p) => match option_p {
            Some(option_p) => format!("{}.proto", option_p.name),
            None => return HttpResponse::NotFound().finish(),
        },
        Err(_) => return HttpResponse::NotFound().finish(),
    };

    let current_proto_file = vc.read_file_content(&proto_file_name).await.unwrap();

    let mut parser = ProtoParser::new();
    let parsed = match parser.parse(&current_proto_file) {
        Ok(proto_file) => proto_file,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let proto_content = match service.add_to_proto_context(parsed) {
        Err(_) => return HttpResponse::InternalServerError().finish(),
        Ok(p) => p,
    };

    vc.save_version(
        &proto_file_name,
        &proto_content.to_proto_text(),
        &user_id.to_string(),
    )
    .await
    .expect("save error message");

    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        ContractRepository::create_contract(&mut conn, payload)
    })
    .await;

    match result {
        Ok(Ok(_)) => HttpResponse::NoContent().finish(),
        Ok(Err(_)) => HttpResponse::InternalServerError().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
