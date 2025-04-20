use crate::schema::sessions;
use chrono::{NaiveDateTime, Utc};
use diesel::{prelude::*, result::Error};
use uuid::Uuid;

use super::models::{NewSession, Session};

pub struct SessionRepository;

impl SessionRepository {
    /// Создает новую сессию пользователя
    pub fn create(conn: &mut PgConnection, new_session: &NewSession) -> Result<Session, Error> {
        diesel::insert_into(sessions::table)
            .values(new_session)
            .returning(Session::as_returning())
            .get_result(conn)
    }

    /// Находит активную сессию по токену и IP
    pub fn find_valid_by_token(
        conn: &mut PgConnection,
        session_token: &str,
        ip_address: &str,
    ) -> Result<Session, Error> {
        use crate::schema::sessions::dsl::*;

        sessions
            .filter(token.eq(session_token))
            .filter(ip_address.eq(ip_address))
            .filter(is_active.eq(true))
            .filter(expires_at.gt(Utc::now().naive_utc()))
            .first(conn)
    }

    /// Деактивирует все сессии пользователя
    pub fn deactivate_all_for_user(conn: &mut PgConnection, user_id: i32) -> Result<usize, Error> {
        use crate::schema::sessions::dsl::*;

        diesel::update(sessions.filter(user_id.eq(user_id)))
            .set(is_active.eq(false))
            .execute(conn)
    }

    /// Деактивирует конкретную сессию
    pub fn deactivate_session(conn: &mut PgConnection, session_id: Uuid) -> Result<usize, Error> {
        use crate::schema::sessions::dsl::*;

        diesel::update(sessions.find(session_id))
            .set(is_active.eq(false))
            .execute(conn)
    }

    /// Получает все активные сессии пользователя
    pub fn find_active_sessions(
        conn: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<Vec<Session>, Error> {
        use crate::schema::sessions::dsl::*;

        sessions
            .filter(user_id.eq(user_id))
            .filter(is_active.eq(true))
            .filter(expires_at.gt(Utc::now().naive_utc()))
            .load(conn)
    }

    /// Обновляет время жизни сессии
    pub fn refresh_session(
        conn: &mut PgConnection,
        session_id: Uuid,
        new_expires_at: NaiveDateTime,
    ) -> Result<Session, Error> {
        use crate::schema::sessions::dsl::*;

        diesel::update(sessions.find(session_id))
            .set(expires_at.eq(new_expires_at))
            .returning(Session::as_returning())
            .get_result(conn)
    }

    // /// Очищает просроченные сессии
    // pub fn cleanup_expired_sessions(conn: &mut PgConnection) -> Result<usize, Error> {
    //     use crate::schema::sessions::dsl::*;
    //     // diesel::delete(rustaceans::table.find(id)).execute(c).await

    //     diesel::delete(sessions.filter(expires_at.lt(Utc::now()))).execute(conn)
    // }
}
