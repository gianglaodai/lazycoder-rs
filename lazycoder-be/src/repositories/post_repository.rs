use crate::services::post_service::Post;
use sqlx::{PgPool, query, query_as};
use crate::define_orm_with_common_fields;

#[derive(Clone)]
pub struct PostRepository {
    pool: PgPool,
}

define_orm_with_common_fields!(PostOrm {title: String,body: String,});

impl From<PostOrm> for Post {
    fn from(orm: PostOrm) -> Self {
        Self {
            id: orm.id,
            uid: orm.uid,
            created_at: orm.created_at,
            updated_at: orm.updated_at,
            title: orm.title,
            body: orm.body,
        }
    }
}

impl PostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_posts(&self) -> Result<Vec<Post>, sqlx::Error> {
        let rows: Vec<PostOrm> = query_as::<_, PostOrm>("select id, title, body from posts")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(Post::from).collect())
    }

    pub async fn find_post_by_id(&self, post_id: i32) -> Result<Post, sqlx::Error> {
        let row: PostOrm =
            query_as::<_, PostOrm>("select id, title, body from posts where id = $1")
                .bind(post_id)
                .fetch_one(&self.pool)
                .await?;
        Ok(Post::from(row))
    }

    pub async fn create_post(&self, post: &Post) -> Result<Post, sqlx::Error> {
        let row: PostOrm = query_as::<_, PostOrm>(
            "insert into posts (title, body) values ($1, $2) returning id, title, body",
        )
        .bind(&post.title)
        .bind(&post.body)
        .fetch_one(&self.pool)
        .await?;
        Ok(Post::from(row))
    }

    pub async fn update_post(&self, post: &Post) -> Result<Post, sqlx::Error> {
        let row: PostOrm = query_as::<_, PostOrm>(
            "update posts set title = $2, body = $3 where id = $1 returning id, title, body",
        )
        .bind(post.id.unwrap())
        .bind(&post.title)
        .bind(&post.body)
        .fetch_one(&self.pool)
        .await?;
        Ok(Post::from(row))
    }

    pub async fn delete_post(&self, post_id: i32) -> Result<u64, sqlx::Error> {
        let result = query("delete from posts where id = $1")
            .bind(post_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}
