use actix_web::{HttpResponse, Responder, web};
use askama::Template;

use crate::{auth::middleware::AuthenticatedUser, storage::db::DbPool, user::model::User};

use super::{model::ProjectCollaborators, repository::ProjectRepository};

#[derive(Template)]
#[template(path = "pages/project/project.html")]
pub struct ProjectTemplate {
    username: String,
    user_photo_url: String,
}

#[derive(Template)]
#[template(path = "pages/project/components/drawer.html")]
pub struct ProjectDrawerTemplate {
    pub nav_items: Vec<NavItem>,
    pub current_path: String,
}

#[derive(Template)]
#[template(path = "pages/project/components/create-project-form.html")]
pub struct CreateProjectTemplate {}

#[derive(Template)]
#[template(path = "pages/project/components/aside.html")]
pub struct ProjectAsideTemplate {
    collaborators: Vec<User>,
}

pub struct NavItem {
    pub id: String,
    pub link: String,
    pub name: String,
}

pub async fn index(a_user: AuthenticatedUser) -> impl Responder {
    let template = ProjectTemplate {
        user_photo_url: a_user.0.photo_url.unwrap_or_else(|| String::from("")),
        username: a_user.0.name,
    };

    match template.render() {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn render_create_project_form() -> impl Responder {
    let template = CreateProjectTemplate {};
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

pub async fn render_drawer(pool: web::Data<DbPool>) -> impl Responder {
    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        ProjectRepository::find_all(&mut conn)
    })
    .await;
    let items = match result {
        Ok(Ok(projects)) => projects
            .into_iter()
            .map(|project| NavItem {
                id: project.id.to_string(),
                link: format!("/project/{}/contracts", project.id),
                name: project.name.clone(),
            })
            .collect::<Vec<NavItem>>(),
        _ => Vec::new(),
    };

    let template = ProjectDrawerTemplate {
        current_path: format!("project"),
        nav_items: items,
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}

pub async fn render_aside(pool: web::Data<DbPool>) -> impl Responder {
    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        ProjectRepository::get_project_collaborators(&mut conn, 1)
    })
    .await;
    let collaborators = match result {
        Ok(Ok(collaborators)) => collaborators,
        _ => ProjectCollaborators {
            collaborators: Vec::new(),
        },
    };
    let template = ProjectAsideTemplate {
        collaborators: collaborators.collaborators,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap())
}
