use chrono::Utc;
use frunk::labelled::Transmogrifier;

use crate::core::{
    domain::{
        auth_helper::{
            JwtSigner, check_password, generate_refresh_token, get_unverified_user_id,
            hash_refresh_token,
        },
        errors::DomainError,
    },
    models::auth::{
        AuthenticatedResponse, LoginRequest, LogoutRequest, TokenRefreshRequest, TokenResponse,
    },
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

        let refresh_tokens = generate_refresh_token(&user.id, self.config.auth_refresh_ttl());

        self.db.create_refresh_token(refresh_tokens.1)?;

        Ok(AuthenticatedResponse {
            auth: TokenResponse {
                access_token,
                refresh_token: refresh_tokens.0,
            },
            user: user.transmogrify(),
        })
    }

    fn refresh_auth_tokens(
        &self,
        refresh_req: TokenRefreshRequest,
    ) -> Result<TokenResponse, DomainError> {
        let hashed_old_token = &hash_refresh_token(&refresh_req.refresh_token);

        let old_token = self
            .db
            .get_refresh_token(hashed_old_token)
            .map_err(|_| DomainError::BadRequest)?;

        if Utc::now().naive_utc() >= old_token.expires_on {
            return Err(DomainError::BadRequest);
        }

        let user_id = old_token.user_id;
        let user = self
            .db
            .get_user(&user_id)
            .map_err(|_| DomainError::BadRequest)?;

        let signer = JwtSigner::new(&self.config.auth_secret(), &user.jwt_secret);
        let access_token = signer.create_token(&user.id, self.config.auth_ttl())?;

        let refresh_tokens = generate_refresh_token(&user_id, self.config.auth_refresh_ttl());
        self.db.create_refresh_token(refresh_tokens.1)?;

        // delete old token
        self.db
            .delete_refresh_token(hashed_old_token)
            .map_err(|_| DomainError::BadRequest)?;

        Ok(TokenResponse {
            access_token,
            refresh_token: refresh_tokens.0,
        })
    }

    fn logout(&self, logout_req: LogoutRequest) -> Result<(), DomainError> {
        let hashed_token = &hash_refresh_token(&logout_req.refresh_token);

        self.db.delete_refresh_token(hashed_token)?;

        Ok(())
    }
}
