use crate::model::user::UserState;
use crate::model::credentail::VarifyPasswordState;
use crate::repo::database::mongodb::MongoRepo;
use crate::model::token::{Token, TokenAuthType};

use actix_web::{
    post,
    error::ResponseError,
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

#[derive(Debug, Display)]
pub enum PasswordError {
    IncorrectPassword,
    AccountLocked,
    ServerError,
    BadRequest,
    UserDoesntExist,
}

#[derive(Deserialize, Serialize)]
pub struct PasswordPost{
    user_name: String,
    password: String,
}

impl ResponseError for PasswordError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            PasswordError::AccountLocked => StatusCode::LOCKED,
            PasswordError::IncorrectPassword => StatusCode::FORBIDDEN,
            PasswordError::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
            PasswordError::BadRequest => StatusCode::BAD_REQUEST,
            PasswordError::UserDoesntExist => StatusCode::NOT_FOUND,
        }
    }

}

#[post("/password")]
pub async fn varify_password (
    mut payload: Payload,
    mongo_repo: Data<MongoRepo>,
) -> Result<Json<Token>, PasswordError> {

    let mut body = BytesMut::new();
    while let Some(chunk) = payload.next().await {
        body.extend_from_slice(&chunk.unwrap());
    }

    let obj_result = serde_json::from_slice::<PasswordPost>(&body);

    if obj_result.is_err() {
        return Err(PasswordError::BadRequest)
    }

    let request = obj_result.unwrap();

    let user_option = mongo_repo.get_user_by_user_name(request.user_name.clone()).await;

    if user_option.is_none() {
        return Err(PasswordError::UserDoesntExist);
    }

    let mut user = user_option.unwrap();

    if user.user_state == UserState::Disabled {
        return Err(PasswordError::AccountLocked);
    }

    let credentail = mongo_repo.get_credentail(user.user_uuid.clone()).await;

    if  credentail.is_none() {
        return Err(PasswordError::ServerError);
    }

    let password_verifiaction = credentail.as_ref().unwrap().varify_password(request.password.clone());

    if password_verifiaction.state == VarifyPasswordState::Failed || password_verifiaction.state == VarifyPasswordState::FailedPreviousPassword {
        return Err(PasswordError::IncorrectPassword);
    }

    let token_res = Token::new(user.user_uuid.clone(), 180, user.user_claims.clone(), TokenAuthType::Full);

    if token_res.as_ref().is_err() {
        println!("{}", token_res.as_ref().unwrap_err());
        return Err(PasswordError::ServerError);
    }

    user.login();

    let user_update = mongo_repo.update_user(user.clone()).await;

    if user_update.is_err() {
        return Err(PasswordError::ServerError);
    }

    return Ok(Json(token_res.unwrap()));


}