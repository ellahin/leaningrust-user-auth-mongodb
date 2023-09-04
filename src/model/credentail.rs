use bcrypt;
use totp_rs::{Algorithm, TOTP, Secret};
use bson::DateTime;
use strum_macros::{EnumString, Display};
use serde::{Serialize, Deserialize};
use crate::model::user::User;


#[derive(PartialEq, Serialize, Deserialize, EnumString, Display, Eq, Debug, Clone)]
pub enum UserMfaState {
    None,
    OTP,
}

pub enum VarifyMfaState {
    Failed,
    Success,
    NotConfigured,
}
pub enum VarifyMfaStateError {
    MissingMfaStore,
    MfaTypeNotImplimented,
}

pub enum AddMfaError {
    Failed,
    MfaTypeNotImplimented,
    MfaTypeNone,
}

#[derive(PartialEq, Serialize, Deserialize, EnumString, Display, Eq, Debug, Clone)]
pub enum VarifyPasswordState {
    Success,
    Failed,
    FailedPreviousPassword
}

pub struct VarifyPassword {
    pub state: VarifyPasswordState,
    pub password_set: Option<DateTime>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserCredentailsExistingPasswords {
    password: String,
    changed_date: DateTime
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserCredentail {
    pub user_uuid: String,
    user_password: String,
    pub user_mfa_state: UserMfaState,
    pub user_mfa_store: Option<String>,
    exsting_passwords: Vec<UserCredentailsExistingPasswords>

}


impl UserCredentail {
    pub fn new (
        user: User,
        plain_password: String,
    ) -> UserCredentail {
        return UserCredentail {
            user_uuid: user.user_uuid,
            user_password: bcrypt::hash(plain_password, 10).unwrap(),
            user_mfa_state: UserMfaState::None,
            user_mfa_store: None,
            exsting_passwords: Vec::new()
        };
    }

    pub fn varify_password (&self, plain_password: String) -> VarifyPassword {

        let mut hash_state = bcrypt::verify(&plain_password, &self.user_password).unwrap();
    
        if hash_state {

            return VarifyPassword {
                state: VarifyPasswordState::Success,
                password_set: None
            };
            
        }

        for last_password in &self.exsting_passwords {

            hash_state = bcrypt::verify(&plain_password, &last_password.password).unwrap();

            if hash_state {
                return VarifyPassword {
                    state: VarifyPasswordState::FailedPreviousPassword,
                    password_set: Some(last_password.changed_date)
                };
            }

        }

        return VarifyPassword{
            state: VarifyPasswordState::Failed,
            password_set: None
        };


    }

    pub fn update_password (&mut self, plain_password: String) {

        self.exsting_passwords.push(UserCredentailsExistingPasswords { password: self.user_password.clone(), changed_date: DateTime::now() });

        if self.exsting_passwords.len() >= 11 {
            self.exsting_passwords.remove(0);
        }

        self.user_password = bcrypt::hash(plain_password, 10).unwrap();
    }

    pub fn remove_mfa (&mut self) {

        self.user_mfa_state = UserMfaState::None;
        self.user_mfa_store = None;

    }

    
    pub fn add_mfa (&mut self, mfa_type: UserMfaState) -> Result<String, AddMfaError> {

        if mfa_type == UserMfaState::None {
            return Result::Err(AddMfaError::MfaTypeNone);
        }

        if mfa_type == UserMfaState::OTP {

            let secret = Secret::generate_secret().to_string();

            self.user_mfa_store = Some(secret.clone());
            self.user_mfa_state = UserMfaState::OTP;
            return Result::Ok(secret);

        }

        return Result::Err(AddMfaError::MfaTypeNotImplimented);

    }


    pub fn check_mfa (&mut self, mfa_code: String, submit_time: u64) -> Result<VarifyMfaState, VarifyMfaStateError> {

        if self.user_mfa_state == UserMfaState::None {
            return Result::Ok(VarifyMfaState::NotConfigured);
        }



        if self.user_mfa_store.is_none() {
            return Result::Err(VarifyMfaStateError::MissingMfaStore);
        }

        if self.user_mfa_state == UserMfaState::OTP {

            let totp = TOTP::new(
                Algorithm::SHA1,
                6,
                1,
                30,
                self.user_mfa_store.clone().unwrap().as_bytes().to_vec()
            ).unwrap();

            if totp.check(&mfa_code, submit_time) {
                return Result::Ok(VarifyMfaState::Success);
            } else {
                return Result::Ok(VarifyMfaState::Failed);
            }

        }

        return Result::Err(VarifyMfaStateError::MfaTypeNotImplimented);        

    }

}