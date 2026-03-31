use database_client::models::UserGroupMapping;
use tracing::debug;

use crate::core::{
    domain::errors::DomainError,
    models::{
        auth::UserId,
        group::{CreateGroup, DisplayGroup, DisplayGroupMember, UpdateRequestGroup},
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

    fn update_group(
        &self,
        user_id: UserId,
        id: &str,
        request: UpdateRequestGroup,
    ) -> Result<DisplayGroup, DomainError> {
        let request_owner: Vec<&DisplayGroupMember> = request
            .members
            .iter()
            .filter(|m| m.role == "owner")
            .collect();

        // There may only be one group owner
        if request_owner.len() != 1 {
            debug!("Owner length wrong");
            return Err(DomainError::BadRequest);
        }

        // User is neither owner nor admin
        if !self.db.check_user_can_update_group(id, &user_id.0)? {
            debug!("User {} cannot update group", user_id.0);
            return Err(DomainError::BadRequest);
        }

        let request_owner_id = &request_owner
            .first()
            .expect("Expected there to be one owner after check")
            .id;

        let db_group_members = self
            .db
            .get_members_for_multiple_groups(&[id])?
            .get(id)
            .ok_or(DomainError::BadRequest)?
            .to_owned();

        let db_owner = db_group_members
            .iter()
            .filter(|m| m.role == "owner")
            .collect::<Vec<&DisplayGroupMember>>()
            .first()
            .ok_or(DomainError::BadRequest)?
            .to_owned();

        let db_requesting_user = db_group_members
            .iter()
            .filter(|m| m.id == user_id.0)
            .collect::<Vec<&DisplayGroupMember>>()
            .first()
            .ok_or(DomainError::BadRequest)?
            .to_owned();

        // If user is not group owner but group owner is modified, return Bad Request
        if db_requesting_user.role != "owner" && request_owner_id != &db_owner.id {
            debug!("Non-owner {} tried to modify owner", user_id.0);
            return Err(DomainError::BadRequest);
        }

        let (group, members) = self.db.update_group(id, request)?;

        debug!("Group id {} updated successfully", id);

        Ok(DisplayGroup::from_group(group, members))
    }
}
