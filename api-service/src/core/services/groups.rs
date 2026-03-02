use database_client::models::UserGroupMapping;
use frunk::labelled::Transmogrifier;

use crate::core::{
    domain::errors::DomainError,
    models::{
        auth::UserId,
        group::{CreateGroup, DisplayGroup},
    },
    ports::service::ServiceGroupPort,
    services::Service,
};

impl ServiceGroupPort for Service {
    fn create_group(
        &self,
        user_id: UserId,
        new_group: CreateGroup,
    ) -> Result<DisplayGroup, DomainError> {
        let group = self.db.create_group(new_group.into())?;
        let mapping = UserGroupMapping {
            user_id: user_id.0,
            group_id: group.id.clone(),
            role_id: "owner".to_string(),
        };
        self.db.create_user_group_mapping(mapping)?;

        Ok(group.transmogrify())
    }
    fn list_groups_by_user(&self, user_id: UserId) -> Result<Vec<DisplayGroup>, DomainError> {
        let groups = self.db.list_groups_by_user_id(&user_id.0)?;
        Ok(groups
            .into_iter()
            .map(frunk::labelled::Transmogrifier::transmogrify)
            .collect())
    }
}
