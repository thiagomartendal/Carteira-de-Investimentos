use sqlx::PgPool;

use crate::modules::account::models::UserRecord;

pub struct Repository {
    db: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            db: pool
        }
    }

    pub async fn add_user(&self, email: &str, username: &str, password_hash: &str, user_type: i32) -> sqlx::Result<UserRecord> {
        sqlx::query_as!(
            UserRecord,
            "insert into users (email, username, password_hash, user_type) values ($1, $2, $3, $4) returning id, email, username, password_hash, user_type;",
            email,
            username,
            password_hash,
            user_type
        ).fetch_one(&self.db).await
    }

    pub async fn get_user_by_email(&self, email: &str) -> sqlx::Result<Option<UserRecord>> {
        sqlx::query_as!(
            UserRecord,
            "select * from users where email = $1",
            email
        )
        .fetch_optional(&self.db)
        .await
    }
}

#[cfg(test)]
impl From<PgPool> for Repository {
    fn from(db: PgPool) -> Self {
        Self { db }
    }
}
