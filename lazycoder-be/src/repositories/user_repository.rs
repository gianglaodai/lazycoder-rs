use crate::define_orm_with_common_fields;
use crate::services::user_service::User;
use sqlx::{PgPool, query_as, query};

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

define_orm_with_common_fields!(UserOrm {
    username: String,
    email: String,
    password: String
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

    pub async fn find_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let rows: Vec<UserOrm> = query_as::<_, UserOrm>(
            "select id, uid, created_at, updated_at, username, email, password from users",
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(User::from).collect())
    }

    pub async fn find_user_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        Ok(query_as::<_, UserOrm>("select id, uid, created_at, updated_at, username, email, password from users where id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await.map(User::from)?)
    }

    pub async fn creat_user(&self, user: &User) -> Result<User, sqlx::Error> {
        let row = query_as::<_, UserOrm>(
            "insert into users (uid, created_at, updated_at, username, email, password) values ($1, $2, $3, $4, $5, $6) returning id, uid, created_at, updated_at, username, email, password",
        )
        .bind(&user.uid)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password)
        .fetch_one(&self.pool)
        .await?;
        Ok(User::from(row))
    }
    
    pub async fn update_user(&self, user: &User) -> Result<User, sqlx::Error> {
        let row = query_as::<_, UserOrm>(
            "update users set updated_at = $2, username = $3, email = $4, password = $5 where id = $1 returning id, uid, created_at, updated_at, username, email, password",
        )
        .bind(user.id.unwrap())
        .bind(&user.updated_at)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password)
        .fetch_one(&self.pool)
        .await?;
        Ok(User::from(row))
    }
    
    pub async fn delete_user(&self, id: i32) -> Result<u64, sqlx::Error> {
        let result = query("delete from users where id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}
