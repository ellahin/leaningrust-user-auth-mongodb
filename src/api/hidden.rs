use crate::model::token::{Token, TokenAuthType};
use crate::model::claims::ClaimsUserType;

use actix_web::{
    get,
    error::ResponseError,
    web::Path,
    HttpResponse,
    HttpRequest,
    http::{header::ContentType, StatusCode, header}
};
use serde::{Serialize, Deserialize};
use strum_macros::Display;
#[derive(Deserialize, Serialize)]
pub struct HiddenPath {
    something: String,
}

#[derive(Debug, Display)]
pub enum GetHiddenError {
    NotAuthorized,
    NoToken,
    MalformedRequest,
}

impl ResponseError for GetHiddenError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            GetHiddenError::NotAuthorized => StatusCode::UNAUTHORIZED,
            GetHiddenError::NoToken => StatusCode::UNAUTHORIZED,
            GetHiddenError::MalformedRequest => StatusCode::BAD_REQUEST,
        }
    }

}


#[get("/hidden/{something}")]
pub async fn get_hidden(
        something: Path<HiddenPath>,
        req: HttpRequest
        ) -> Result<String, GetHiddenError>{
    
    let headders = req.head().headers.clone();

    let auth_header = headders.get(header::AUTHORIZATION);

    if auth_header.is_none() {
        return Err(GetHiddenError::NoToken);
    }

    let auth_string = auth_header.unwrap().to_str();

    if auth_string.is_err() {
        return Err(GetHiddenError::MalformedRequest);
    }

    let mut token = Token::new_from_authorization(auth_string.unwrap().to_string());

    let validation = token.validate_jwt_token();

    if validation.is_err() {
        return Err(GetHiddenError::MalformedRequest);
    }

    if !validation.ok().unwrap() {
        return Err(GetHiddenError::NotAuthorized);
    }

    if token.claims.is_none() {
        return Err(GetHiddenError::MalformedRequest)
    }

    if !(token.claims.clone().unwrap().auth_type == TokenAuthType::Full) {
        return Err(GetHiddenError::NotAuthorized);
    } 

    if token.claims.clone().unwrap().user_claim.user_type == ClaimsUserType::Guest {
        return Err(GetHiddenError::NotAuthorized);
    }

    return Ok(something.into_inner().something)
}