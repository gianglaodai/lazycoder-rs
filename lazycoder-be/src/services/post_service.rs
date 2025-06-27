use crate::infras::repositories::post_repository::PostRepository;
use std::sync::Arc;
use crate::define_struct_with_common_fields;

define_struct_with_common_fields!(Post {
    pub title: String,
    pub body: String,
});

#[derive(Clone)]
pub struct PostService {
    post_repository: Arc<PostRepository>,
}

impl PostService {
    pub fn new(post_repository: Arc<PostRepository>) -> Self {
        Self { post_repository }
    }

    pub async fn get_many(&self) -> Result<Vec<Post>, sqlx::Error> {
        self.post_repository.find_many().await
    }

    pub async fn get_by_id(&self, id: i32) -> Result<Post, sqlx::Error> {
        self.post_repository.find_by_id(id).await
    }

    pub async fn create(&self, post: Post) -> Result<Post, sqlx::Error> {
        self.post_repository.create(&post).await
    }

    pub async fn update(&self, post: Post) -> Result<Post, sqlx::Error> {
        self.post_repository.update(&post).await
    }

    pub async fn delete_by_id(&self, id: i32) -> Result<u64, sqlx::Error> {
        self.post_repository.delete(id).await
    }
}
