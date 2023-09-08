use crate::model::{user::User, credentail::UserCredentail};
use strum_macros::Display;


#[derive(Display, Eq, PartialEq, Debug)]
pub enum DatabaseError {
    UserNameExists,
    UserUuidExists,
    UserDoesntExist,
    DBFailure,
}
pub trait Database {
    async fn init (
        connection_url: String,
        database: String
    ) -> Self;

    async fn get_user(
        &self, 
        user_uudi: String
    ) -> Option<User>;

    async fn get_credentail(
        &self, 
        user_uudi: String
    ) -> Option<UserCredentail>;

    async fn get_user_by_user_name(
        &self, 
        user_name: String
    ) -> Option<User>;

    async fn insert_user(
        &self, 
        user: User
    ) -> Result<User, DatabaseError>;

    async fn insert_credentail(
        &self, 
        credentail: UserCredentail
    ) -> Result<bool, DatabaseError>;

    async fn delete_user(
        &self, 
        user: User
    ) -> Result<User, DatabaseError>;

    async fn update_user(
        &self, 
        user: User
    ) -> Result<User, DatabaseError>;
    
}