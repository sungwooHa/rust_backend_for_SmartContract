use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

use dotenv::dotenv;
use std::env;
use time::{Duration, OffsetDateTime};

use crate::util::jwt_claims;

use super::jwt_claims::Claims;

// pub fn generate_hash_with_time(input: &String) -> String {
//     let timestamp_nanos = Utc::now().timestamp_nanos(); // e.g. `2014-11-28T12:45:59.324310806Z`
//     let hash = Sha256::new()
//         .chain_update(input)
//         .chain_update(timestamp_nanos.to_string())
//         .chain_update(OsRng.next_u64().to_string())
//         .finalize();

//     base64_url::encode(&hash)
// }

pub fn generate_expired_hash(input: &String, expired_duration: Duration) -> String {
    let iat = OffsetDateTime::now_utc();
    let exp = iat + expired_duration;

    let claims = jwt_claims::Claims::new(input.to_string(), iat, exp);

    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").expect("Secret_key must be set");

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .unwrap()
}

pub fn decode_token(token: &String) -> Result<String, String> {
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").expect("Secret_key must be set");
    jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|token_data| token_data.claims.get_sub())
    .map_err(|err| err.to_string())
}

pub fn is_expired_hash(token: &String) -> bool {
    dotenv().ok();
    let secret_key = env::var("SECRET_KEY").expect("Secret_key must be set");

    let token_data = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .expect("fail to decode jwt");

    token_data.claims.is_expired()
}
