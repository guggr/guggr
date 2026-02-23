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
pub fn create_token(
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
    storage.auth().create_refresh_token(new_token)?;

    Ok(TokenResponse {
        access_token: jwt_signed.to_string(),
        refresh_token: refresh_jwt_signed.to_string(),
    })
}

pub fn refresh_token(
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

    let old_record = storage.auth().get_refresh_token(&jti)?;
    let old_user = old_record.user_id.clone();
    let old_meta: AuthMetadata = old_record.transmogrify();
    if meta != old_meta {
        return Err(AuthError::ChangedAuthMetadata);
    }
    let new_token = create_token(signer, storage, meta, &old_user, ttl, ttl_refresh)?;
    invalidate_token(signer, storage, old_token)?;
    Ok(new_token)
}

pub fn invalidate_token(
    signer: &JwsEs256Signer,
    storage: &Arc<dyn StoragePort>,
    token: &str,
) -> Result<(), AuthError> {
    let jwt = verify_jwt(signer, token)?;
    if let Some(jti) = jwt.jti {
        storage.auth().delete_refresh_token(&jti)?;
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use argon2::{
        PasswordHasher,
        password_hash::{SaltString, rand_core::OsRng},
    };

    use super::*;
    use crate::core::ports::storage::tests::MockStore;

    #[actix_web::test]
    async fn validate_token() -> anyhow::Result<()> {
        let signer = JwsEs256Signer::generate_es256()?;
        let storage: Arc<dyn StoragePort> = Arc::new(MockStore::new());
        let meta = AuthMetadata {
            ip_address: "0.0.0.0".to_string(),
            user_agent: "bogus".to_string(),
        };
        let ttl = 60;
        let ttl_refresh = 600;
        let user_id = "bob";
        let token = create_token(&signer, &storage, meta.clone(), user_id, ttl, ttl_refresh)?;
        let v = verify_jwt(&signer, &token.access_token)?;
        let a = signer.sign(&v)?;
        assert_eq!(token.access_token, a.to_string());
        assert!(
            refresh_token(
                &signer,
                &storage,
                AuthMetadata::default(),
                &token.refresh_token,
                ttl,
                ttl_refresh,
            )
            .is_ok()
        );
        assert!(invalidate_token(&signer, &storage, &token.refresh_token).is_ok());
        Ok(())
    }

    #[actix_web::test]
    async fn jwt_expired() -> anyhow::Result<()> {
        let signer = JwsEs256Signer::generate_es256()?;
        let storage: Arc<dyn StoragePort> = Arc::new(MockStore::new());
        let meta = AuthMetadata {
            ip_address: "0.0.0.0".to_string(),
            user_agent: "bogus".to_string(),
        };
        let ttl = 0;
        let ttl_refresh = 0;
        let user_id = "bob";
        let token = create_token(&signer, &storage, meta.clone(), user_id, ttl, ttl_refresh)?;
        actix_web::rt::time::sleep(Duration::from_secs(1)).await;
        assert_eq!(
            verify_jwt(&signer, &token.access_token).unwrap_err(),
            AuthError::JwtExpired
        );
        assert_eq!(
            refresh_token(
                &signer,
                &storage,
                AuthMetadata::default(),
                &token.refresh_token,
                ttl,
                ttl_refresh,
            )
            .unwrap_err(),
            AuthError::JwtExpired
        );
        Ok(())
    }

    #[actix_web::test]
    async fn jti_missing_from_refresh() -> anyhow::Result<()> {
        let signer = JwsEs256Signer::generate_es256()?;
        let storage: Arc<dyn StoragePort> = Arc::new(MockStore::new());
        let meta = AuthMetadata {
            ip_address: "0.0.0.0".to_string(),
            user_agent: "bogus".to_string(),
        };
        let ttl = 0;
        let ttl_refresh = 0;
        let user_id = "bob";
        let token = create_token(&signer, &storage, meta.clone(), user_id, ttl, ttl_refresh)?;
        assert_eq!(
            refresh_token(
                &signer,
                &storage,
                AuthMetadata::default(),
                // we just use the access token here, as that does not have the jti claim set
                &token.access_token,
                ttl,
                ttl_refresh,
            )
            .unwrap_err(),
            AuthError::JtiMissing
        );
        Ok(())
    }

    #[test]
    fn validate_passwords() -> anyhow::Result<()> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let s = "secret".to_string();
        let h = argon2
            .hash_password(s.as_bytes(), &salt)
            .unwrap()
            .to_string();
        assert!(verify_password(&s, &h)?);
        assert!(!verify_password("othersecret", &h)?);
        Ok(())
    }
}
