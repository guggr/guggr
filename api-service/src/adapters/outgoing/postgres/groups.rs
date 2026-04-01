use std::collections::HashMap;

use database_client::{
    models::{Group, UserGroupMapping},
    schema::{group, user, user_group_mapping},
};
use diesel::{ExpressionMethods, NullableExpressionMethods, QueryDsl, RunQueryDsl};
use tracing::debug;

use crate::{
    adapters::outgoing::postgres::{Postgres, PostgresError},
    core::{
        domain::errors::DomainError,
        models::group::{DisplayGroupMember, UpdateRequestGroup},
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

    fn get_group(&self, id: &str) -> Result<Group, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let group = group::dsl::group
            .find(id)
            .first(&mut conn)
            .map_err(PostgresError::from)?;

        Ok(group)
    }

    fn list_groups_by_user_id(
        &self,
        user_id: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<database_client::models::Group>, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let groups: Vec<Group> = group::table
            .inner_join(user_group_mapping::table)
            .filter(user_group_mapping::user_id.eq(user_id))
            .select((group::id, group::name))
            .distinct()
            .order(group::name.asc())
            .limit(limit)
            .offset(offset)
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

    fn check_user_can_update_group(&self, id: &str, user_id: &str) -> Result<bool, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let role: Option<String> = user_group_mapping::table
            .select(user_group_mapping::role_id.nullable())
            .filter(user_group_mapping::group_id.eq(id))
            .filter(user_group_mapping::user_id.eq(user_id))
            .first(&mut conn)
            .map_err(PostgresError::from)?;

        Ok(role
            .map(|r| ["owner", "admin"].contains(&r.as_str()))
            .unwrap_or_default())
    }

    fn update_group(
        &self,
        group_id: &str,
        updated_group: UpdateRequestGroup,
    ) -> Result<(Group, Vec<DisplayGroupMember>), DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;

        let new_group = diesel::update(group::table.find(group_id))
            .set(group::name.eq(updated_group.name))
            .get_result(&mut conn)
            .map_err(PostgresError::from)?;

        debug!("Group id {} name updated successfully", group_id);

        let new_mappings: Vec<_> = updated_group
            .members
            .into_iter()
            .map(|m| {
                (
                    user_group_mapping::group_id.eq(group_id),
                    user_group_mapping::user_id.eq(m.id),
                    user_group_mapping::role_id.eq(m.role),
                )
            })
            .collect();

        let mut new_members: Vec<UserGroupMapping> = Vec::new();

        conn.build_transaction()
            .serializable()
            .run(|conn| {
                diesel::delete(user_group_mapping::table)
                    .filter(user_group_mapping::group_id.eq(group_id))
                    .execute(conn)?;

                new_members = diesel::insert_into(user_group_mapping::table)
                    .values(new_mappings)
                    .get_results(conn)?;

                diesel::result::QueryResult::Ok(())
            })
            .map_err(PostgresError::from)?;

        debug!("Group id {} mappings deleted", group_id);

        let member_names: Vec<(String, String)> = user::table
            .select((user::id, user::name))
            .filter(
                user::id.eq_any(
                    new_members
                        .iter()
                        .map(|m| &m.user_id)
                        .collect::<Vec<&String>>(),
                ),
            )
            .order_by(user::id.asc())
            .load(&mut conn)
            .map_err(PostgresError::from)?;

        debug!("Loaded member names for group id {}", group_id);

        new_members.sort_by(|a, b| a.user_id.cmp(&b.user_id));

        let new_members: Vec<DisplayGroupMember> = new_members
            .into_iter()
            .zip(member_names)
            .map(|(m, (_, name))| DisplayGroupMember {
                id: m.user_id,
                name,
                role: m.role_id,
            })
            .collect();

        debug!("Performed member mapping for display in result");

        Ok((new_group, new_members))
    }

    fn count_groups(&self, user_id: &str) -> Result<i64, DomainError> {
        let mut conn = self.pool.get().map_err(PostgresError::from)?;
        let count: i64 = group::table
            .inner_join(user_group_mapping::table)
            .filter(user_group_mapping::user_id.eq(user_id))
            .count()
            .get_result(&mut conn)
            .map_err(PostgresError::from)?;
        Ok(count)
    }
}
