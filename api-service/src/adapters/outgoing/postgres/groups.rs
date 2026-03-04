use std::collections::HashMap;

use database_client::{
    models::Group,
    schema::{group, user, user_group_mapping},
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    adapters::outgoing::postgres::{Postgres, PostgresError},
    core::{
        domain::errors::DomainError, models::group::DisplayGroupMember,
        ports::repository::RepositoryGroupPort,
    },
};

impl RepositoryGroupPort for Postgres {
    fn create_group(
        &self,
        new_group: database_client::models::Group,
    ) -> Result<database_client::models::Group, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let result: Group = diesel::insert_into(group::dsl::group)
            .values(new_group)
            .get_result(&mut conn)
            .map_err(PostgresError::from)?;

        Ok(result)
    }

    fn list_group_ids_by_user_id(
        &self,
        user_id: &str,
    ) -> Result<Vec<database_client::models::Group>, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let groups: Vec<Group> = group::table
            .inner_join(user_group_mapping::table)
            .filter(user_group_mapping::user_id.eq(user_id))
            .select((group::id, group::name))
            .distinct()
            .order(group::name.asc())
            .load::<Group>(&mut conn)
            .map_err(PostgresError::from)?;

        Ok(groups)
    }

    fn get_members_for_multiple_groups(
        &self,
        group_ids: &[&str],
    ) -> Result<HashMap<String, Vec<DisplayGroupMember>>, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let all_members: Vec<(String, (String, String, String))> = user::table
            .inner_join(user_group_mapping::table)
            .filter(user_group_mapping::group_id.eq_any(group_ids))
            .select((
                user_group_mapping::group_id,
                (user::id, user::name, user_group_mapping::role_id),
            ))
            .load(&mut conn)
            .map_err(PostgresError::from)?;

        // Group the results by group_id for easy lookup
        let mut map: HashMap<String, Vec<DisplayGroupMember>> = HashMap::new();
        for m in all_members {
            map.entry(m.0.clone())
                .or_default()
                .push(DisplayGroupMember {
                    id: m.1.0,
                    name: m.1.1,
                    role: m.1.2,
                });
        }

        Ok(map)
    }
}
