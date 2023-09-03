
use bson::DateTime;
use strum_macros::{EnumString, Display};
use serde::{Serialize, Deserialize};
use uuid::Uuid;


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
}

impl User {
    pub fn new (
        user_email: String,
    ) -> User {
        User {
            user_uuid: Uuid::new_v4().to_string(),
            user_email,
            user_state: UserState::NotActivated,
            last_login: DateTime::now(),
        }
    }

    pub fn login(&mut self) {
        self.last_login = DateTime::now();
    }
}

