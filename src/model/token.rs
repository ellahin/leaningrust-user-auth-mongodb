use serde::{Deserialize, Serialize};
use uuid::Uuid;
use jsonwebtoken::Validation;
use crate::model::claims::Claims;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Token {
    pub token: Option<String>,
    pub token_uuid: Option<String>,
    pub user_id: Option<String>,
    pub expires_in: Option<i64>,
    pub claims: Option<TokenClaims>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TokenAuthType {
    Full,
    RequiresMFA,
    RequiresValidation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub sub: String,
    pub token_uuid: String,
    pub user_claim: Claims,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
    pub auth_type: TokenAuthType
}

pub enum ValidateError {
    NoToken,
    TokenNotValid,
}


impl Token {

    pub fn new_from_authorization(
        authorization: String
    ) -> Token {
        return Token { 
            token: Some(authorization), 
            token_uuid: None, 
            user_id: None, 
            expires_in: None, 
            claims: None, 
        }
    }

    pub fn new(
        user_id: String,
        ttl: i64,
        user_claims: Claims,
        auth_type: TokenAuthType
    ) -> Result<Token, jsonwebtoken::errors::Error> {
    
        let now = chrono::Utc::now();

        let mut token_details = Token {
            user_id: Some(user_id),
            token_uuid: Some(Uuid::new_v4().to_string()),
            expires_in: Some((now + chrono::Duration::minutes(ttl)).timestamp()),
            token: None,
            claims: None,
        };
    
        let claims = TokenClaims {
            sub: token_details.user_id.as_ref().unwrap().to_string(),
            token_uuid: token_details.token_uuid.as_ref().unwrap().to_string(),
            user_claim: user_claims,
            exp: token_details.expires_in.clone().unwrap(),
            iat: now.timestamp(),
            nbf: now.timestamp(),
            auth_type
        };
    
        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::ES256);
        let token = jsonwebtoken::encode(
            &header,
            &claims,
            &jsonwebtoken::EncodingKey::from_ec_pem(include_bytes!("../../private.pem"))?,
        )?;
        token_details.token = Some(token);
        token_details.claims = Some(claims);
        return Ok(token_details);
    }

    pub fn validate_jwt_token(&mut self) -> Result<bool, ValidateError> {

        let validation = Validation::new(jsonwebtoken::Algorithm::ES256);

        if self.token.is_none() {
            return Err(ValidateError::NoToken)
        }

        let token = self.token.clone().unwrap();

        let token_data = jsonwebtoken::decode::<TokenClaims>(
            &token, 
            &jsonwebtoken::DecodingKey::from_ec_pem(include_bytes!("../../public.pem")).unwrap(), 
            &validation);

        if token_data.is_ok() {
            self.claims = Some(token_data.clone().unwrap().claims);
            self.user_id = Some(token_data.clone().unwrap().claims.user_claim.user_name);
            self.token_uuid = Some(token_data.clone().unwrap().claims.token_uuid);
            return Ok(true);
        }

        println!("{:?}", token_data.err());

        return Err(ValidateError::TokenNotValid);
        
    }

}