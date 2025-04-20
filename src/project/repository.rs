use super::model::{NewProject, Project, ProjectCollaborators, UpdateProject};
use crate::schema::projects;
use crate::schema::projects::dsl::*;
use crate::user::model::User;

use diesel::prelude::*;
use diesel::result::Error;

pub struct ProjectRepository;

impl ProjectRepository {
    pub fn create(conn: &mut PgConnection, new_project: NewProject) -> Result<Project, Error> {
        diesel::insert_into(projects::table)
            .values(new_project)
            .get_result(conn)
    }

    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Project>, Error> {
        projects.load::<Project>(conn)
    }

    pub fn find_by_id(conn: &mut PgConnection, project_id: i32) -> Result<Option<Project>, Error> {
        projects.filter(id.eq(project_id)).first(conn).optional()
    }

    pub fn update(
        conn: &mut PgConnection,
        project_id: i32,
        update_data: UpdateProject,
    ) -> Result<Project, Error> {
        diesel::update(projects.find(project_id))
            .set(&update_data)
            .get_result(conn)
    }

    pub fn delete(conn: &mut PgConnection, project_id: i32) -> Result<usize, Error> {
        diesel::delete(projects.find(project_id)).execute(conn)
    }

    pub fn get_project_collaborators(
        conn: &mut PgConnection,
        project_id: i32,
    ) -> Result<ProjectCollaborators, Error> {
        use crate::schema::collaborators::dsl::*;
        use crate::schema::collaborators::*;
        use crate::schema::users;
        use crate::schema::users::dsl::*;

        #[derive(Queryable)]
        struct Collaborator {
            pub id: i32,
            pub name: String,
            pub email: String,
        }

        let project_collaborators: Vec<Collaborator> = users
            .inner_join(collaborators.on(user_id.eq(users::id)))
            .select((users::id, users::name, users::email)) // Replace with actual column names
            .distinct()
            .load(conn)?;

        let project_collaborators = project_collaborators
            .into_iter()
            .map(|c| User {
                id: c.id,
                name: c.name,
                email: c.email,
                ..Default::default()
            })
            .collect();

        Ok(ProjectCollaborators {
            collaborators: project_collaborators,
        })
    }
}
