use database_client::models::UserGroupMapping;

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

        let group_id = group.id.clone();
        let members = self.db.get_members_for_multiple_groups(&[&group_id])?;

        Ok(DisplayGroup::from_group(
            group,
            members.get(&group_id).cloned().unwrap_or_default(),
        ))
    }

    fn get_group(&self, user_id: UserId, id: &str) -> Result<DisplayGroup, DomainError> {
        let group = self.db.get_group(id)?;
        let members = self.db.get_members_for_multiple_groups(&[id])?;

        let group = DisplayGroup::from_group(group, members.get(id).cloned().unwrap_or_default());

        // Return not found if user is not part of group
        if !group.members.iter().any(|member| member.id == user_id.0) {
            return Err(DomainError::NotFound);
        }

        Ok(group)
    }

    fn list_groups_by_user(&self, user_id: UserId) -> Result<Vec<DisplayGroup>, DomainError> {
        let groups = self.db.list_groups_by_user_id(&user_id.0)?;

        let group_ids: Vec<&str> = groups.iter().map(|g| g.id.as_str()).collect();

        let members_map = self.db.get_members_for_multiple_groups(&group_ids)?;

        let display_groups = groups
            .into_iter()
            .map(|g| {
                let members = members_map.get(&g.id).cloned().unwrap_or_default();
                DisplayGroup::from_group(g, members)
            })
            .collect();

        Ok(display_groups)
    }
}
