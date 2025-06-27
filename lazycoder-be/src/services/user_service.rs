use std::sync::Arc;
use crate::define_struct_with_common_fields;
use crate::infras::repositories::user_repository::UserRepository;

define_struct_with_common_fields!(User {
    pub username: String,
    pub email: String,
    pub password: String
});

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<UserRepository>,
}

impl UserService {
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self {
            user_repository,
        }
    }
    
    pub async fn get_many(&self) -> Result<Vec<User>, sqlx::Error> {
        self.user_repository.find_many().await
    }
    
    pub async fn get_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        self.user_repository.find_by_id(id).await
    }
    
    pub async fn create(&self, user: User) -> Result<User, sqlx::Error> {
        self.user_repository.create(&user).await
    }
    
    pub async fn update(&self, user: User) -> Result<User, sqlx::Error> {
        self.user_repository.update(&user).await
    }
    
    pub async fn delete(&self, id: i32) -> Result<u64, sqlx::Error> {
        self.user_repository.delete(id).await
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
       self.user_repository.find_by_email(email).await
    }

    pub async fn get_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
       self.user_repository.find_by_username(username).await
    }
}