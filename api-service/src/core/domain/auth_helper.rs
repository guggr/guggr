use std::str::FromStr;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;

use crate::core::domain::errors::AuthError;

pub fn verify_password(password: &str, phc_hash: &str) -> Result<bool, AuthError> {
    let parsed = PasswordHash::new(phc_hash).map_err(|_| AuthError::InvalidHashformat)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

use compact_jwt::{
    JwsEs256Signer, JwsSigner, JwsSignerToVerifier, JwsVerifier, Jwt, JwtUnverified,
};

pub fn create_jwt(signer: &JwsEs256Signer, user_id: &str, ttl_s: i64) -> Result<String, AuthError> {
    let now = Utc::now().timestamp();
    let jwt: Jwt<()> = Jwt::<()> {
        sub: Some(user_id.to_string()),
        exp: Some(now + ttl_s),
        iat: Some(now),
        ..Default::default()
    };

    let signed = signer.sign(&jwt)?;
    Ok(signed.to_string())
}

pub fn verify_jwt(signer: &JwsEs256Signer, token: &str) -> Result<Jwt<()>, AuthError> {
    let now = Utc::now().timestamp();
    let unverified = JwtUnverified::from_str(token)?;
    let verifier = signer.get_verifier()?;
    let verified = verifier.verify(&unverified)?;

    if let Some(expiry) = verified.exp
        && expiry < now
    {
        return Err(AuthError::JwtExpired);
    }

    Ok(verified)
}
