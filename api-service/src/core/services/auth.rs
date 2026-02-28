use std::ops::Add;

use chrono::{Duration, Utc};
use database_client::models::RefreshToken;
use frunk::labelled::Transmogrifier;

use crate::core::{
    domain::{
        auth_helper::{
            JwtSigner, check_password, get_unverified_user_id, hash_and_encode_refresh_token,
        },
        errors::DomainError,
    },
    models::auth::{AuthenticatedResponse, LoginRequest, TokenResponse},
    ports::service::ServiceAuthPort,
    services::Service,
};

impl ServiceAuthPort for Service {
    fn validate_access_token(&self, token: &str) -> Result<String, DomainError> {
        let user_id = get_unverified_user_id(token).map_err(|_| DomainError::Unauthorized)?;

        let user = self
            .db
            .get_user(&user_id)
            .map_err(|_| DomainError::Unauthorized)?;

        let signer = JwtSigner::new(&self.config.auth_secret(), &user.jwt_secret);
        signer
            .verify_access_token(token)
            .map_err(|_| DomainError::Unauthorized)?;

        Ok(user_id)
    }

    fn login(&self, login_req: LoginRequest) -> Result<AuthenticatedResponse, DomainError> {
        let Ok(user) = self.db.get_user_by_email(&login_req.email) else {
            return Err(DomainError::BadRequest);
        };

        if !check_password(&login_req.password, &user.password) {
            return Err(DomainError::BadRequest);
        }

        let signer = JwtSigner::new(&self.config.auth_secret(), &user.jwt_secret);
        let access_token = signer.create_token(&user.id, self.config.auth_ttl())?;

        let refresh_token = nanoid::nanoid!(32);

        let refresh_token_db = RefreshToken {
            token: hash_and_encode_refresh_token(&refresh_token),
            user_id: user.id.clone(),
            expires_on: Utc::now()
                .naive_utc()
                .add(Duration::seconds(self.config.auth_refresh_ttl())),
        };

        self.db.create_refresh_token(refresh_token_db)?;

        Ok(AuthenticatedResponse {
            auth: TokenResponse {
                access_token,
                refresh_token,
            },
            user: user.transmogrify(),
        })
    }
}
