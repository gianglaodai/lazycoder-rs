use std::sync::Arc;
use crate::define_struct_with_common_fields;
use crate::repositories::user_repository::UserRepository;

define_struct_with_common_fields!(User {
    username: String,
    email: String,
    password: String
});

#[derive(Clone)]
pub struct UserService {
    user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self {
            user_repository,
        }
    }
    
    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        self.user_repository.find_users().await
    }
    
    pub async fn get_user_by_id(&self, id: i32) -> Result<User, sqlx::Error> {
        self.user_repository.find_user_by_id(id).await
    }
    
    pub async fn create_user(&self, user: User) -> Result<User, sqlx::Error> {
        self.user_repository.creat_user(&user).await
    }
    
    pub async fn update_user(&self, user: User) -> Result<User, sqlx::Error> {
        self.user_repository.update_user(&user).await
    }
    
    pub async fn delete_user(&self, id: i32) -> Result<u64, sqlx::Error> {
        self.user_repository.delete_user(id).await
    }
}