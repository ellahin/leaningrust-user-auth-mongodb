use crate::model::{user::User, credentail::UserCredentail};
use crate::repo::database::base::DatabaseError;
use crate::repo::database::base::Database as BaseDatabase;

use mongodb::{Client, options::ClientOptions, Database, bson::doc};

#[derive(Clone)]
pub struct MongoRepo {
    client_database: Database,
}

impl BaseDatabase for MongoRepo {

    async fn init (
        connection_url: String,
        database: String
    ) -> MongoRepo {
        let client_options = ClientOptions::parse(&connection_url).await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        return MongoRepo{
            client_database: client.database(database.as_str())
        }
    }

    async fn get_user(&self, user_uudi: String) -> Option<User> {

        let collection = self.client_database.collection::<User>("users");

        let user = collection.find_one(doc! {"user_email": user_uudi.clone()}, None).await;

        if user.is_err() {
            return None;
        }

        if user.as_ref().ok().is_none(){
            return None;
        }

        if user.as_ref().unwrap().is_none() {
            return None;
        }

        return Some(user.unwrap().unwrap());

    }


    async fn get_credentail(&self, user_uudi: String) -> Option<UserCredentail> {

        let collection = self.client_database.collection::<UserCredentail>("credentails");

        let credentail = collection.find_one(doc! {"user_uuid": &user_uudi}, None).await;

        if credentail.is_err() {
            return None;
        }

        if credentail.as_ref().ok().is_none(){
            return None;
        }

        if credentail.as_ref().unwrap().is_none() {
            return None;
        }

        return Some(credentail.unwrap().unwrap());

    }


    async fn get_user_by_user_name(&self, user_name: String) -> Option<User> {

        let collection = self.client_database.collection::<User>("users");

        let user = collection.find_one(doc! {"user_email": &user_name}, None).await;

        if user.is_err() {
            return None;
        }

        if user.as_ref().ok().is_none(){
            return None;
        }

        if user.as_ref().unwrap().is_none() {
            return None;
        }

        return Some(user.unwrap().unwrap());

    }

    async fn insert_user(&self, user: User) -> Result<User, DatabaseError> {

        let collection = self.client_database.collection::<User>("users");

        let mut db_find = self.get_user(user.user_uuid.clone()).await;

        if db_find.is_some(){
            return Err(DatabaseError::UserUuidExists);
        }

        
        
        db_find = self.get_user_by_user_name(user.user_email.clone()).await;

        if db_find.is_some(){
            return Err(DatabaseError::UserNameExists);
        }

        let insert = collection.insert_one(user.clone(), None).await;

        if insert.is_err() {
            return Err(DatabaseError::DBFailure);
        } 
            
        return Ok(user);

    }


    async fn insert_credentail(&self, credentail: UserCredentail) -> Result<bool, DatabaseError> {

        let collection = self.client_database.collection::<UserCredentail>("credentails");

        let db_find = self.get_credentail(credentail.user_uuid.clone()).await;

        if db_find.is_some(){
            return Err(DatabaseError::UserUuidExists);
        }

        let insert = collection.insert_one(credentail.clone(), None).await;

        if insert.is_err() {
            return Err(DatabaseError::DBFailure);
        } 
            
        return Ok(true);

    }

    async fn delete_user(&self, user: User) -> Result<User, DatabaseError> {

        let collection = self.client_database.collection::<User>("users");

        let user_find = self.get_user(user.user_uuid.clone()).await;

        if user_find.is_none() {
            return Err(DatabaseError::UserDoesntExist);
        }

        let delete = collection.delete_one(doc! {"user_uuid": user.user_uuid.clone()}, None).await;

        if delete.as_ref().is_err() {
            return Err(DatabaseError::DBFailure)
        }

        return Ok(user)

    }

    async fn update_user(&self, user: User) -> Result<User, DatabaseError> {

        let collection = self.client_database.collection::<User>("users");

        let user_insert = collection.update_one(
            doc!{"user_uuid": user.user_uuid.clone()}, 
            doc! {"$set": bson::to_bson(&user).unwrap()}, 
            None
        ).await;

        if user_insert.is_err() {
            return Err(DatabaseError::DBFailure);
        }

        return Ok(user);
    }



}