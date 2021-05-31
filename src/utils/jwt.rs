use jsonwebtoken::{
    decode as jwt_decode, encode as jwt_encode, errors, Algorithm, DecodingKey, EncodingKey,
    Header, Validation,
};

use crate::models::user::Claims;

#[allow(dead_code)]
pub fn encode(user: Claims, secret: &[u8]) -> Result<String, errors::Error> {
    jwt_encode(&Header::default(), &user, &EncodingKey::from_secret(secret))
}

pub fn decode(token: String, secret: &[u8]) -> Result<Claims, errors::Error> {
    Ok(jwt_decode::<Claims>(
        token.as_str(),
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )?
    .claims)
}
