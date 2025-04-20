use diesel::PgConnection;
use diesel::prelude::*;

use super::model::NewUser;
use super::model::User;

pub struct UserRepository;

impl UserRepository {
    pub fn find_or_create_oauth_user(_conn: &mut PgConnection, _user_info: ()) -> User {
        User::default()
    }

    pub fn find_by_id(conn: &mut PgConnection, user_id: i32) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        users.find(user_id).first(conn)
    }

    pub fn find_by_email(conn: &mut PgConnection, user_email: &str) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        users.filter(email.eq(user_email)).first(conn)
    }

    pub fn create(conn: &mut PgConnection, new_user: &NewUser) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        diesel::insert_into(users)
            .values(new_user)
            .returning(User::as_returning())
            .get_result(conn)
    }

    pub fn update(conn: &mut PgConnection, user: User) -> QueryResult<User> {
        use crate::schema::users::dsl::*;
        diesel::update(users.find(user.id))
            .set((photo_url.eq(user.photo_url),))
            .get_result(conn)
    }
}
