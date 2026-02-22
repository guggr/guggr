use std::{str::FromStr, sync::Arc};

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use frunk::labelled::Transmogrifier;

use crate::core::{
    domain::errors::AuthError,
    models::auth::{AuthMetadata, CreateRefreshToken, TokenResponse},
    ports::storage::StoragePort,
};

pub fn verify_password(password: &str, phc_hash: &str) -> Result<bool, AuthError> {
    let parsed = PasswordHash::new(phc_hash).map_err(|_| AuthError::InvalidHashformat)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

use compact_jwt::{
    JwsEs256Signer, JwsSigner, JwsSignerToVerifier, JwsVerifier, Jwt, JwtUnverified,
};
pub async fn create_token(
    signer: &JwsEs256Signer,
    storage: &Arc<dyn StoragePort>,
    meta: AuthMetadata,
    user_id: &str,
    ttl: i64,
    ttl_refresh: i64,
) -> Result<TokenResponse, AuthError> {
    let now = Utc::now().timestamp();
    let jwt: Jwt<()> = Jwt::<()> {
        sub: Some(user_id.to_string()),
        exp: Some(now + ttl),
        iat: Some(now),
        ..Default::default()
    };
    let refresh_jwt_jti = nanoid::nanoid!(32);
    let refresh_jwt_exp = now + ttl_refresh;
    let refresh_jwt: Jwt<()> = Jwt::<()> {
        sub: Some(user_id.to_string()),
        exp: Some(refresh_jwt_exp),
        iat: Some(now),
        jti: Some(refresh_jwt_jti.clone()),
        ..Default::default()
    };

    let jwt_signed = signer.sign(&jwt)?;
    let refresh_jwt_signed = signer.sign(&refresh_jwt)?;

    let new_token = CreateRefreshToken {
        jti: refresh_jwt_jti,
        user_id: user_id.to_string(),
        ip_address: meta.ip_address,
        user_agent: meta.user_agent,
        expires_on: refresh_jwt_exp,
    };
    storage.auth().create_refresh_token(new_token).await?;

    Ok(TokenResponse {
        access_token: jwt_signed.to_string(),
        refresh_token: refresh_jwt_signed.to_string(),
    })
}

pub async fn refresh_token(
    signer: &JwsEs256Signer,
    storage: &Arc<dyn StoragePort>,
    meta: AuthMetadata,
    old_token: &str,
    ttl: i64,
    ttl_refresh: i64,
) -> Result<TokenResponse, AuthError> {
    let jti = verify_jwt(signer, old_token)?
        .jti
        .ok_or(AuthError::JtiMissing)?;

    let old_record = storage.auth().get_refresh_token(&jti).await?;
    let old_user = old_record.user_id.clone();
    let old_meta: AuthMetadata = old_record.transmogrify();
    if meta != old_meta {
        return Err(AuthError::ChangedAuthMetadata);
    }
    let new_token = create_token(signer, storage, meta, &old_user, ttl, ttl_refresh).await?;
    invalidate_token(signer, storage, old_token).await?;
    Ok(new_token)
}

pub async fn invalidate_token(
    signer: &JwsEs256Signer,
    storage: &Arc<dyn StoragePort>,
    token: &str,
) -> Result<(), AuthError> {
    let jwt = verify_jwt(signer, token)?;
    if let Some(jti) = jwt.jti {
        storage.auth().delete_refresh_toke(&jti).await?;
    }
    Ok(())
}

pub fn verify_jwt(signer: &JwsEs256Signer, token: &str) -> Result<Jwt<()>, AuthError> {
    let now = Utc::now().timestamp();
    let jwt_unverified = JwtUnverified::from_str(token)?;
    let verifier = signer.get_verifier()?;
    let jwt_verified = verifier.verify(&jwt_unverified)?;

    if let Some(expiry) = jwt_verified.exp
        && expiry < now
    {
        return Err(AuthError::JwtExpired);
    }

    Ok(jwt_verified)
}
