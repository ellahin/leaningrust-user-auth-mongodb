
use bson::DateTime;
use strum_macros::{EnumString, Display};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::model::claims::{Claims, ClaimsUserType};

#[derive(PartialEq, Serialize, Deserialize, EnumString, Display, Eq, Clone)]
pub enum UserState {
    Active,
    Disabled,
    NoSubscription,
    NotActivated,
    VarifyingMFA,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub user_uuid: String,
    pub user_email: String,
    pub user_state: UserState,
    pub last_login: DateTime,
    pub user_claims: Claims,
}

impl User {
    pub fn new (
        user_email: String,

    ) -> User {

        let uuid = Uuid::new_v4().to_string();
        User {
            user_uuid: uuid.clone(),
            user_email: user_email.clone(),
            user_state: UserState::NotActivated,
            last_login: DateTime::now(),
            user_claims: Claims{
                user_type: ClaimsUserType::User,
                user_uuid: uuid.clone(),
                user_name: user_email,
                group_uuid: Vec::new(),
            }
        }
    }

    pub fn login(&mut self) {
        self.last_login = DateTime::now();
    }
}

