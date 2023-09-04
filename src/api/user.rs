use crate::model::user::User;
use crate::model::credentail::UserCredentail;
use crate::repo::mongodb::{MongoRepo, MongoErrors};
use actix_web::{
    get,
    post,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    web::Payload,
    web::BytesMut,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use futures_util::StreamExt;
use serde::{Serialize, Deserialize};
use strum_macros::Display;
use serde_json;


#[derive(Deserialize, Serialize)]
pub struct UserUuid {
    user_uuid: String
}
#[derive(Deserialize, Serialize)]
pub struct NewUser {
    user_name: String,
    password: String
}
#[derive(Debug, Display)]
pub enum NewUserError {
    ServerFailure,
    UserAlreadyExists,
    BadRequest,
}

#[derive(Debug, Display)]
pub enum UserGetError {
    NotFound,
}

impl ResponseError for UserGetError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            UserGetError::NotFound => StatusCode::NOT_FOUND,
        }
    }

}

impl ResponseError for NewUserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            NewUserError::ServerFailure => StatusCode::INTERNAL_SERVER_ERROR,
            NewUserError::UserAlreadyExists => StatusCode::CONFLICT,
            NewUserError::BadRequest => StatusCode::BAD_REQUEST,
        }
    }

}

#[get("/user/{user_uuid}")]
pub async fn get_user(
        user_uuid: Path<UserUuid>,
        mongo_repo: Data<MongoRepo>,
        ) -> Result<Json<User>, UserGetError>{
    
    let user = mongo_repo.get_user(user_uuid.into_inner().user_uuid).await;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(UserGetError::NotFound)
    }
}

#[post("/new/user")]
pub async fn new_user (
    mut payload: Payload,
    mongo_repo: Data<MongoRepo>,
) -> Result<Json<User>, NewUserError> {

    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        body.extend_from_slice(&chunk.unwrap());
    }

    // body is loaded, now we can deserialize serde-json
    let obj_result = serde_json::from_slice::<NewUser>(&body);

    if obj_result.is_err(){
        return Err(NewUserError::BadRequest)
    }

    let user = obj_result.unwrap();

    let user_exists = mongo_repo.get_user_by_user_name(user.user_name.clone()).await;

    if user_exists.is_some() {
        return Err(NewUserError::UserAlreadyExists);
    }

    let mut user_obj: User = User::new(user.user_name.clone());

    let mut user_insert_status = mongo_repo.insert_user(user_obj.clone()).await;

    if user_insert_status.is_err() {

        while user_insert_status.as_ref().err() == Some(&MongoErrors::UserUuidExists) {
            user_obj = User::new(user.user_name.clone());
            user_insert_status = mongo_repo.insert_user(user_obj.clone()).await;
        }

    }

    drop(user_obj);

    if user_insert_status.is_err() {
        match user_insert_status.as_ref().err().unwrap() {
            MongoErrors::UserNameExists => return Err(NewUserError::UserAlreadyExists),
            MongoErrors::DBFailure => return Err(NewUserError::ServerFailure),
            MongoErrors::UserDoesntExist => return Err(NewUserError::ServerFailure),
            MongoErrors::UserUuidExists => return Err(NewUserError::ServerFailure),
        }
    }

    let user_db_obj = user_insert_status.unwrap();

    let credentail_obj: UserCredentail = UserCredentail::new(user_db_obj.clone(), user.password.clone());

    let credentail_insert_status = mongo_repo.insert_credentail(credentail_obj).await;

    if credentail_insert_status.as_ref().is_err() {
        let _ = mongo_repo.delete_user(user_db_obj.clone()).await;

        return Err(NewUserError::BadRequest);
    }

    return Ok(Json(user_db_obj));    

}

