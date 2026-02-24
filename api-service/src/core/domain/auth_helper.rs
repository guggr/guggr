use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use chrono::Utc;
use config::ApiServiceConfig;
use hkdf::Hkdf;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, dangerous::insecure_decode, decode,
    encode,
};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::core::{
    domain::errors::AuthError,
    models::auth::{CreateRefreshToken, TokenResponse},
    ports::storage::StoragePort,
};

#[derive(Serialize, Deserialize, Debug)]
struct Claims {
    sub: String,
    iat: i64,
    exp: i64,
}

impl Claims {
    pub fn new(sub: &str, ttl: i64) -> Self {
        let now = Utc::now().timestamp();
        Self {
            sub: sub.to_string(),
            iat: now,
            exp: now + ttl,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct RefreshClaims {
    sub: String,
    iat: i64,
    exp: i64,
    jti: String,
}

impl RefreshClaims {
    pub fn new(sub: &str, ttl: i64) -> Self {
        let now = Utc::now().timestamp();
        Self {
            sub: sub.to_string(),
            iat: now,
            exp: now + ttl,
            jti: nanoid::nanoid!(32),
        }
    }
}

pub fn verify_password(password: &str, phc_hash: &str) -> Result<bool, AuthError> {
    let parsed = PasswordHash::new(phc_hash).map_err(|_| AuthError::InvalidHashformat)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

pub fn get_unverified_user(token: &str) -> Result<String, AuthError> {
    let unverified_claims = insecure_decode::<Claims>(token)?;
    Ok(unverified_claims.claims.sub)
}

pub struct JwtSigner {
    main: Vec<u8>,
    salt: Vec<u8>,
    user_secret: Vec<u8>,
}

impl JwtSigner {
    pub fn new(global: &[u8], salt: &[u8], user_secret: &[u8]) -> Self {
        Self {
            main: global.to_vec(),
            salt: salt.to_vec(),
            user_secret: user_secret.to_vec(),
        }
    }
    fn derive_user_key(&self) -> [u8; 32] {
        let hk = Hkdf::<Sha256>::new(Some(&self.salt), &self.main);
        let mut out = [0; 32];
        hk.expand(&self.user_secret, &mut out).expect("aaa");
        out
    }

    fn get_encoding_key(&self) -> EncodingKey {
        let uk = self.derive_user_key();
        EncodingKey::from_secret(&uk)
    }

    fn get_decoding_key(&self) -> DecodingKey {
        let uk = self.derive_user_key();
        DecodingKey::from_secret(&uk)
    }

    pub fn create_token(
        &self,
        user_id: &str,
        config: &ApiServiceConfig,
        storage: &Arc<dyn StoragePort>,
    ) -> Result<TokenResponse, AuthError> {
        let ek = self.get_encoding_key();
        let header = Header::new(Algorithm::HS256);
        let access_claims = Claims::new(user_id, config.auth_ttl());
        let refresh_claims = RefreshClaims::new(user_id, config.auth_refresh_ttl());
        let new_refresh_token = CreateRefreshToken {
            jti: refresh_claims.jti.clone(),
            user_id: refresh_claims.sub.clone(),
            expires_on: refresh_claims.exp,
        };
        storage.auth().create_refresh_token(new_refresh_token)?;

        Ok(TokenResponse {
            access_token: encode(&header, &access_claims, &ek)?,
            refresh_token: encode(&header, &refresh_claims, &ek)?,
        })
    }

    pub fn refresh_token(
        &self,
        config: &ApiServiceConfig,
        storage: &Arc<dyn StoragePort>,
        old_token: &str,
    ) -> Result<TokenResponse, AuthError> {
        let dk = self.get_decoding_key();
        let jti = decode::<RefreshClaims>(old_token, &dk, &Validation::new(Algorithm::HS256))?
            .claims
            .jti;
        let old_record = storage.auth().get_refresh_token(&jti)?;
        let old_user = old_record.user_id.clone();
        let new_token = self.create_token(&old_user, config, storage)?;
        self.invalidate_token(storage, old_token)?;
        Ok(new_token)
    }

    pub fn invalidate_token(
        &self,
        storage: &Arc<dyn StoragePort>,
        old_token: &str,
    ) -> Result<(), AuthError> {
        let dk = self.get_decoding_key();
        let jti = decode::<RefreshClaims>(old_token, &dk, &Validation::new(Algorithm::HS256))?
            .claims
            .jti;
        storage.auth().delete_refresh_token(&jti)?;
        Ok(())
    }

    pub fn verify_access_token(&self, token: &str) -> Result<(), AuthError> {
        let dk = self.get_decoding_key();
        decode::<Claims>(&token, &dk, &Validation::new(Algorithm::HS256))?;
        Ok(())
    }
}
