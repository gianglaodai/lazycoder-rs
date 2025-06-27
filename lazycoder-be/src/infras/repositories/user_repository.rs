use crate::define_orm_with_common_fields;
use crate::services::user_service::User;
use sqlx::{PgPool, query_as, query};

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

define_orm_with_common_fields!(UserOrm {
    pub username: String,
    pub email: String,
    pub password: String
});

impl From<UserOrm> for User {
    fn from(user: UserOrm) -> Self {
        Self {
            id: user.id,
            uid: user.uid,
            created_at: user.created_at,
            updated_at: user.updated_at,
            username: user.username,
            email: user.email,
            password: user.password,
        }
    }
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_many(&self) -> Result<Vec<User>, sqlx::Error> {
        let rows: Vec<UserOrm> = query_as::<_, UserOrm>(
            "select * from users",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(User::from).collect())
    }

    pub async fn find_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        Ok(query_as::<_, UserOrm>("select * from users where id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await.map(User::from)?)
    }

    pub async fn create(&self, user: &User) -> Result<User, sqlx::Error> {
        let current = time::OffsetDateTime::now_utc();
        let row = query_as::<_, UserOrm>(
            "insert into users (uid, created_at, updated_at, username, email, password) values ($1, $2, $3, $4, $5, $6) returning *",
        )
        .bind(uuid::Uuid::now_v7())
        .bind(current)
        .bind(current)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password)
        .fetch_one(&self.pool)
        .await?;
        Ok(User::from(row))
    }

    pub async fn update(&self, user: &User) -> Result<User, sqlx::Error> {
        let row = query_as::<_, UserOrm>(
            "update users set updated_at = $2, username = $3, email = $4, password = $5 where id = $1 returning *",
        )
        .bind(user.id.unwrap())
        .bind(time::OffsetDateTime::now_utc())
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password)
        .fetch_one(&self.pool)
        .await?;
        Ok(User::from(row))
    }

    pub async fn delete(&self, id: i32) -> Result<u64, sqlx::Error> {
        let result = query("delete from users where id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
        let row = query_as::<_, UserOrm>("select * from users where username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(User::from))
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let row = query_as::<_, UserOrm>("select * from users where email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(User::from))
    }
}
