pub mod auth;
pub mod contract;
pub mod file_storage;
pub mod project;
pub mod schema;
pub mod storage;
pub mod swagger_parser;
pub mod user;
pub mod utils;

use actix_files::Files;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;
use actix_web::web::Data;
use actix_web::{App, HttpResponse, HttpServer, web};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;

use crate::contract::repository::VersionControl;
use crate::storage::db;

use crate::auth::controller as auth_controller;
use crate::contract::controller as contract_controller;
use crate::project::controller as project_controller;
use crate::user::controller as user_controller;

use crate::contract::web as contract_render;
use crate::project::web as project_render;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    const ENVS: [&'static str; 3] = [
        "/Users/annakarenina/develop/github.com/Anna-Karenina/open_contract/file-storage/.env",
        "/Users/annakarenina/develop/github.com/Anna-Karenina/open_contract/api/.env",
        "/Users/annakarenina/develop/github.com/Anna-Karenina/open_contract/db/.env",
    ];

    ENVS.iter().for_each(|path| {
        dotenvy::from_filename(path).expect("Environment file not provided");
    });

    let minio = file_storage::establish_connection().await;
    let vc = VersionControl::new(Arc::new(minio));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(vc.clone()))
            .app_data(Data::new(db::establish_connection().clone()))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key())
                    .cookie_secure(false)
                    .build(),
            )
            .service(
                web::scope("/project")
                    .route("", web::get().to(project_render::index))
                    .route(
                        "components/create-new-project",
                        web::get().to(project_render::render_create_project_form),
                    )
                    .route(
                        "/components/drawer",
                        web::get().to(project_render::render_drawer),
                    )
                    .route(
                        "/components/aside",
                        web::get().to(project_render::render_aside),
                    )
                    .route(
                        "/{project_id}/contract",
                        web::get().to(contract_render::index),
                    )
                    .route(
                        "/{project_id}/contract/components/tab-services",
                        web::get().to(contract_render::service),
                    )
                    .route(
                        "/{project_id}/contract/components/tab-services/{service_id}/comments",
                        web::get().to(contract_render::comments),
                    )
                    .route(
                        "/{project_id}/contract/components/tab-import",
                        web::get().to(contract_render::import),
                    )
                    .route(
                        "/{project_id}/contract/components/tab-editor",
                        web::get().to(contract_render::editor),
                    ),
            )
            .service(
                web::scope("/api/auth")
                    .route("/login", web::post().to(auth_controller::login))
                    .route("/logout", web::put().to(auth_controller::logout))
                    .route("/get_me", web::get().to(auth_controller::get_me)),
            )
            .service(web::scope("/api/users").route(
                "/avatar",
                web::post().to(user_controller::upload_user_avatar),
            ))
            .service(
                web::scope("/api/project")
                    .route("", web::post().to(project_controller::create_project))
                    .route("", web::get().to(project_controller::get_all_projects))
                    .service(
                        web::resource("/{project_id}")
                            .get(project_controller::get_project)
                            .put(project_controller::update_project)
                            .delete(project_controller::delete_project),
                    )
                    .service(
                        web::scope("/{project_id}/contract")
                            .route("", web::post().to(project_controller::crate_contract)),
                    ),
            )
            .service(
                web::scope("/api/comments")
                    .route("", web::post().to(contract_controller::create_comments)),
            )
            .service(
                web::scope("/api/files")
                    .route(
                        "/{filename}/save",
                        web::post().to(contract_controller::save_version),
                    )
                    .route(
                        "/{filename}/versions",
                        web::get().to(contract_controller::get_versions),
                    )
                    .route(
                        "/{filename}/version/{version_id}",
                        web::get().to(contract_controller::get_version_content),
                    )
                    .route(
                        "/{filename}/diff/{version1}/{version2}",
                        web::get().to(contract_controller::diff_versions),
                    ),
            )
            .service(
                Files::new("/static", "templates/static")
                    .show_files_listing()
                    .use_last_modified(true)
                    .prefer_utf8(true),
            )
            .route(
                "/",
                web::get().to(|| async {
                    HttpResponse::TemporaryRedirect()
                        .append_header(("Location", "/project"))
                        .finish()
                }),
            )
    })
    .bind("127.0.0.1:6969")?
    .run()
    .await
}

fn secret_key() -> Key {
    let key = env::var("SECRET_KEY").expect("SECRET_KEY must be set in .env");
    Key::from(key.as_bytes())
}
