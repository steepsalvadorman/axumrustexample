use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct LoginRequest{
    pub email: String,
    pub password: String,
}


#[derive(Serialize)]
pub struct LoginResponse{
    pub success: bool,
    pub message: String,
    pub token: Option<String>
}


