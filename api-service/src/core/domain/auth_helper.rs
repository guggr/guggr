use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use base64::{Engine, engine::general_purpose};
use chrono::Utc;
use config::ApiServiceConfig;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, dangerous::insecure_decode, decode,
    encode,
};
use serde::{Deserialize, Serialize};
use sha3::Digest;

use crate::core::{
    domain::errors::AuthError,
    models::auth::{CreateRefreshToken, TokenResponse},
    ports::storage::StoragePort,
};

/// JWT Claims for the access token
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

/// Internal Structure containing the token and its hash
#[derive(Serialize, Deserialize, Debug)]
struct RefreshToken {
    pub token: String,
    pub hash: String,
}

impl RefreshToken {
    pub fn new() -> Result<Self, AuthError> {
        let token = nanoid::nanoid!(32);
        Ok(Self {
            token: token.clone(),
            hash: hash_and_encode_refresh_token(&token),
        })
    }
}

/// hahes and base64 encode the supplied token
fn hash_and_encode_refresh_token(token: &str) -> String {
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(token);
    let token_hash = hasher.finalize();
    general_purpose::STANDARD.encode(token_hash)
}

/// compares the supplied password with the supplied argon hash
///
/// # Errors
/// [`AuthError::InvalidHashformat`] if the supplied password hash has an
/// invalid format
pub fn verify_password(password: &str, phc_hash: &str) -> Result<bool, AuthError> {
    let parsed = PasswordHash::new(phc_hash).map_err(|_| AuthError::InvalidHashformat)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

/// gets the **unverified** user id. The JWT is **NOT** verified at this point
///
/// # Errors
/// [`AuthError::JwtError`] if the supplied token can't be decoded
pub fn get_unverified_user_id(token: &str) -> Result<String, AuthError> {
    let unverified_claims = insecure_decode::<Claims>(token)?;
    Ok(unverified_claims.claims.sub)
}

/// JWT Signer
pub struct JwtSigner {
    /// key that is made out of `global-secret || user-secret`
    key: Vec<u8>,
}

impl JwtSigner {
    pub fn new(global: &[u8], user_secret: &[u8]) -> Self {
        Self {
            key: [global, user_secret].concat(),
        }
    }

    fn get_encoding_key(&self) -> EncodingKey {
        EncodingKey::from_secret(&self.key)
    }

    fn get_decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(&self.key)
    }

    fn get_validation(&self) -> Validation {
        let mut v = Validation::new(Algorithm::HS256);
        v.leeway = 10; // still valid after 10s (account for clock skew)
        v
    }

    /// Create and sign a new JWT Access and Refresh token
    pub fn create_token(
        &self,
        user_id: &str,
        config: &ApiServiceConfig,
        storage: &Arc<dyn StoragePort>,
    ) -> Result<TokenResponse, AuthError> {
        let ek = self.get_encoding_key();
        let header = Header::new(Algorithm::HS256);
        let access_claims = Claims::new(user_id, config.auth_ttl());
        let refresh_token = RefreshToken::new()?;
        let new_refresh_token = CreateRefreshToken {
            token: refresh_token.hash,
            user_id: user_id.to_string(),
            expires_on: Utc::now().timestamp() + config.auth_refresh_ttl(),
        };
        storage.auth().create_refresh_token(new_refresh_token)?;

        Ok(TokenResponse {
            access_token: encode(&header, &access_claims, &ek)?,
            refresh_token: refresh_token.token,
        })
    }

    /// Verify an access token
    pub fn verify_access_token(&self, token: &str) -> Result<(), AuthError> {
        let dk = self.get_decoding_key();
        decode::<Claims>(&token, &dk, &self.get_validation())?;
        Ok(())
    }
}

/// Invalidate an older Refresh token
pub fn invalidate_token(storage: &Arc<dyn StoragePort>, old_token: &str) -> Result<(), AuthError> {
    storage
        .auth()
        .delete_refresh_token(&hash_and_encode_refresh_token(old_token))?;
    Ok(())
}

/// Get a new Access and Refresh token from an old Refresh token
pub fn refresh_token(
    config: &ApiServiceConfig,
    storage: &Arc<dyn StoragePort>,
    old_token: &str,
) -> Result<TokenResponse, AuthError> {
    let old_record = storage
        .auth()
        .get_refresh_token(&hash_and_encode_refresh_token(old_token))?;
    if old_record.expires_on >= Utc::now().timestamp() {
        invalidate_token(storage, old_token)?;
        return Err(AuthError::Unauthorized);
    }
    let old_user = old_record.user_id.clone();
    let jwt_secret = storage.auth().get_user_jwt_secrets(&old_user)?;
    let signer = JwtSigner::new(&config.auth_secret(), &jwt_secret.jwt_secret);
    let new_token = signer.create_token(&old_user, config, storage)?;
    invalidate_token(storage, old_token)?;
    Ok(new_token)
}

#[cfg(test)]
mod tests {

    use argon2::{
        PasswordHasher,
        password_hash::{SaltString, rand_core::OsRng},
    };
    use jsonwebtoken::errors::ErrorKind;

    use super::*;
    use crate::core::ports::storage::tests::MockStore;

    #[test]
    fn valid_token() -> anyhow::Result<()> {
        let env_vars = vec![
            ("API_SERVICE_HOST", Some("localhost")),
            ("API_SERVICE_PORT", Some("8000")),
            ("API_SERVICE_AUTH_TTL", Some("60")),
            ("API_SERVICE_AUTH_REFRESH_TTL", Some("6000")),
            ("API_SERVICE_AUTH_SECRET", Some("very-secret")),
        ];
        let config = temp_env::with_vars(env_vars, || ApiServiceConfig::from_env().unwrap());
        let signer = JwtSigner::new(&config.auth_secret(), "secret".as_bytes());
        let storage: Arc<dyn StoragePort> = Arc::new(MockStore::new());

        let token = signer.create_token("cool-user", &config, &storage)?;
        assert!(refresh_token(&config, &storage, &token.refresh_token).is_ok(),);
        signer.verify_access_token(&token.access_token)?;
        assert!(signer.verify_access_token(&token.access_token).is_ok(),);
        Ok(())
    }

    #[test]
    fn jwt_expired() -> anyhow::Result<()> {
        let env_vars = vec![
            ("API_SERVICE_HOST", Some("localhost")),
            ("API_SERVICE_PORT", Some("8000")),
            ("API_SERVICE_AUTH_TTL", Some("-11")),
            ("API_SERVICE_AUTH_REFRESH_TTL", Some("-11")),
            ("API_SERVICE_AUTH_SECRET", Some("very-secret")),
        ];
        let config = temp_env::with_vars(env_vars, || ApiServiceConfig::from_env().unwrap());
        let signer = JwtSigner::new(&config.auth_secret(), "secret".as_bytes());
        let storage: Arc<dyn StoragePort> = Arc::new(MockStore::new());

        let token = signer.create_token("cool-user", &config, &storage)?;
        assert!(matches!(
            signer.verify_access_token(&token.access_token).unwrap_err(),
            AuthError::JwtError(e) if matches!(e.kind(), ErrorKind::ExpiredSignature)
        ));
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
