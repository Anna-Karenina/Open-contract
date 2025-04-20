use crate::{
    auth::middleware::AuthenticatedUser,
    contract::{
        model::{CrateContract, ServiceRequest},
        repository::{ContractRepository, VersionControl},
    },
    storage::db::DbPool,
    swagger_parser, utils,
};
use actix_web::{HttpResponse, Responder, web};
use diesel::result::Error;

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
) -> impl Responder {
    let service = service.into_inner();
    let project_id = project_id.into_inner();

    let body = match serde_json::to_string(&service.body) {
        Ok(it) => Some(it),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let parsed_response = match serde_json::to_string(&service.response) {
        Ok(it) => Some(it),
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    let (parsed_path, parsed_query_params) =
        swagger_parser::parse_utils::parse_path_template(&service.path);
    let grpc_method = format!(
        "{}{}",
        utils::strings_utils::capitalize(&service.method),
        parsed_path
    );
    let parsed_query_params = parsed_query_params.join(": string,\n");
    let payload: CrateContract = CrateContract {
        project_id,
        author_id: a_user.0.id,
        grpc_method,
        tag: Some(service.tag),
        errors_response: Some("errors".to_string()),
        path: Some(service.path),
        query: Some("{\n" + &service.path_params + &parsed_query_params + "§}"),
        body,
        response: parsed_response,
        http_method: Some(service.method),
        description: Some(service.description),
    };
    dbg!(&payload);

    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        ContractRepository::create_contract(&mut conn, payload)
    })
    .await;

    // let json_string = serde_json::to_string(&project).unwrap();
    // let mut file = File::create("data.json").expect("Could not create file!");
    // file.write_all(json_string.as_bytes())
    //     .expect("Cannot write to the file!");

    dbg!(&result);
    match result {
        Ok(Ok(_)) => HttpResponse::NoContent().finish(),
        Ok(Err(Error::NotFound)) => HttpResponse::NotFound().finish(),
        Ok(Err(_)) => HttpResponse::InternalServerError().finish(),
        Err(e) => {
            dbg!(&e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
