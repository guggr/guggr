use std::sync::Arc;

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
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
    domain::errors::{AuthError, DomainError},
    models::auth::TokenResponse,
    ports::storage::StoragePort,
};

/// Hashes the supplied password and returns the Argon2id PHC string.
pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

/// Generates the 16 byte JWT user secret with a CSPRNG.
pub fn generate_user_jwt_secret() -> Vec<u8> {
    // TODO implement CSPRNG usage
    vec![]
}

/// Compares the supplied password with the supplied Argon2 PHC string.
///
/// Returns `false` if any error occurs.
pub fn check_password(password: &str, phc_hash: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(phc_hash) else {
        return false;
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

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

/// Hashes and base64 encodes the supplied token.
pub fn hash_and_encode_refresh_token(token: &str) -> String {
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(token);

    let token_hash = hasher.finalize();
    general_purpose::STANDARD.encode(token_hash)
}

/// Returns the **unverified** user ID. The JWT's validity is **NOT** verified
/// at this point!
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

    /// Creates a new signed JWT access token.
    pub fn create_token(&self, user_id: &str, ttl: i64) -> Result<String, DomainError> {
        let ek = self.get_encoding_key();
        let header = Header::new(Algorithm::HS256);

        let access_claims = Claims::new(user_id, ttl);

        encode(&header, &access_claims, &ek).map_err(|e| DomainError::Internal(e.to_string()))
    }

    /// Verifies the JWT access token.
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
    _config: &ApiServiceConfig,
    _storage: &Arc<dyn StoragePort>,
    _old_token: &str,
) -> Result<TokenResponse, AuthError> {
    // let old_record = storage
    //     .auth()
    //     .get_refresh_token(&hash_and_encode_refresh_token(old_token))?;
    // if old_record.expires_on >= Utc::now().timestamp() {
    //     invalidate_token(storage, old_token)?;
    //     return Err(AuthError::Unauthorized);
    // }
    // let old_user = old_record.user_id.clone();
    // let jwt_secret = storage.auth().get_user_jwt_secrets(&old_user)?;
    // let signer = JwtSigner::new(&config.auth_secret(), &jwt_secret.jwt_secret);
    // let new_token = signer.create_token(&old_user, config, storage)?;
    // invalidate_token(storage, old_token)?;
    // Ok(new_token)

    // TODO
    Err(AuthError::Unauthorized)
}

#[cfg(test)]
mod tests {

    use argon2::{
        PasswordHasher,
        password_hash::{SaltString, rand_core::OsRng},
    };
    use jsonwebtoken::errors::ErrorKind;

    use super::*;

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

        let token = signer.create_token("cool-user", config.auth_ttl())?;
        // assert!(refresh_token(&config, &storage, &token.refresh_token).is_ok(),);

        signer.verify_access_token(&token)?;
        assert!(signer.verify_access_token(&token).is_ok(),);
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

        let token = signer.create_token("cool-user", config.auth_ttl())?;
        assert!(matches!(
            signer.verify_access_token(&token).unwrap_err(),
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
        assert!(check_password(&s, &h));
        assert!(!check_password("othersecret", &h));
        Ok(())
    }
}
