use crate::repositories::post_repository::PostRepository;
use actix_web::web;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    pub id: Option<i32>,
    pub title: String,
    pub body: String,
}

#[derive(Clone)]
pub struct PostService {
    post_repository: Arc<PostRepository>,
}

impl PostService {
    pub fn new(post_repository: Arc<PostRepository>) -> Self {
        Self { post_repository }
    }

    pub async fn get_posts(&self) -> Result<Vec<Post>, sqlx::Error> {
        self.post_repository.find_posts().await
    }

    pub async fn get_post(&self, id: i32) -> Result<Post, sqlx::Error> {
        self.post_repository.find_post(id).await
    }

    pub async fn create_post(&self, post: Post) -> Result<Post, sqlx::Error> {
        self.post_repository.create_post(&post).await
    }

    pub async fn update_post(&self, post: Post) -> Result<Post, sqlx::Error> {
        self.post_repository.update_post(&post).await
    }

    pub async fn delete_post(&self, id: i32) -> Result<u64, sqlx::Error> {
        self.post_repository.delete_post(id).await
    }
}
