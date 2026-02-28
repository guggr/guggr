use chrono::Utc;
use frunk::labelled::Transmogrifier;

use crate::core::{
    domain::{
        auth_helper::{JwtSigner, RefreshToken, check_password, get_unverified_user_id},
        errors::DomainError,
    },
    models::auth::{AuthenticatedResponse, CreateRefreshToken, LoginRequest, TokenResponse},
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

        let refresh_token = RefreshToken::new();
        let _new_refresh_token = CreateRefreshToken {
            token: refresh_token.hash,
            user_id: user.id.clone(),
            expires_on: Utc::now().timestamp() + self.config.auth_refresh_ttl(),
        };

        // TODO persist new_refresh_token

        Ok(AuthenticatedResponse {
            auth: TokenResponse {
                access_token,
                refresh_token: refresh_token.token,
            },
            user: user.transmogrify(),
        })
    }
}
