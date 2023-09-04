use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ClaimsUserType {
    Admin,
    User,
    Guest
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Claims {
    pub user_type: ClaimsUserType,
    pub user_uuid: String,
    pub user_name: String,
    pub group_uuid: Vec<String>,
} 