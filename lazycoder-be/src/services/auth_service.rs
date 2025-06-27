use crate::services::user_service::{User, UserService};
use bcrypt::{DEFAULT_COST, hash, verify};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

#[derive(Debug)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: i64,    // Expiration time
    pub iat: i64,    // Issued at
}

#[derive(Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Clone)]
pub struct AuthService {
    user_service: UserService,
    encoder: Arc<JwtEncoder>,
}

pub struct JwtEncoder {
    pub encode: Box<dyn Fn(&Claims) -> Result<String, String> + Send + Sync>,
}

impl JwtEncoder {
    pub fn encode(&self, claims: &Claims) -> Result<String, String> {
        (self.encode)(&claims)
    }
}

impl AuthService {
    pub fn new(user_service: UserService, encoder: Arc<JwtEncoder>) -> Self {
        Self {
            user_service,
            encoder,
        }
    }

    pub async fn login(&self, login_req: LoginRequest) -> Result<String, String> {
        // Find user by username
        let user_opt = self
            .user_service
            .get_by_username(&login_req.username)
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let user = user_opt.ok_or_else(|| "Invalid username or password".to_string())?;

        // Verify password
        let password_matches = verify(&login_req.password, &user.password)
            .map_err(|_| "Password verification failed".to_string())?;

        if !password_matches {
            return Err("Invalid username or password".to_string());
        }

        // Generate JWT token
        self.generate_token(&user.uid.unwrap().to_string())
    }

    pub async fn register(&self, register_req: RegisterRequest) -> Result<String, String> {
        // Check if username already exists
        if let Ok(Some(_)) = self
            .user_service
            .get_by_username(&register_req.username)
            .await
        {
            return Err("Username already exists".to_string());
        }

        // Check if email already exists
        if let Ok(Some(_)) = self.user_service.get_by_email(&register_req.email).await {
            return Err("Email already exists".to_string());
        }

        // Hash password
        let hashed_password = hash(&register_req.password, DEFAULT_COST)
            .map_err(|e| format!("Password hashing error: {}", e))?;

        // Create new user
        let user = User {
            id: None,
            uid: None, // This will be set by the repository
            created_at: None,
            updated_at: None,
            username: register_req.username,
            email: register_req.email,
            password: hashed_password,
        };

        // Save user to database
        let created_user = self
            .user_service
            .create(user)
            .await
            .map_err(|e| format!("Failed to create user: {}", e))?;

        // Generate JWT token
        self.generate_token(&created_user.uid.unwrap().to_string())
    }

    fn generate_token(&self, user_id: &str) -> Result<String, String> {
        let now = OffsetDateTime::now_utc();
        let expiry = now + Duration::days(7); // Token valid for 7 days

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiry.unix_timestamp(),
            iat: now.unix_timestamp(),
        };

        self.encoder
            .encode(&claims)
            .map_err(|e| format!("Token generation error: {}", e))
    }
}
