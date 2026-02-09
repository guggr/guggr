// @generated automatically by Diesel CLI.

diesel::table! {
    group (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    job (id) {
        id -> Text,
        name -> Text,
        job_type_id -> Text,
        group_id -> Text,
        notify_users -> Bool,
        custom_notification -> Nullable<Text>,
        run_every -> Interval,
        last_scheduled -> Nullable<Timestamp>,
    }
}

diesel::table! {
    job_runs (id) {
        id -> Text,
        job_id -> Text,
        timestamp -> Timestamp,
        triggered_notification -> Bool,
        output -> Nullable<Text>,
        batch_id -> Text,
    }
}

diesel::table! {
    job_type (id) {
        id -> Text,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    role (id) {
        id -> Text,
        name -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Text,
        name -> Text,
        email -> Text,
        password -> Text,
    }
}

diesel::table! {
    user_group_mapping (user_id, group_id) {
        user_id -> Text,
        group_id -> Text,
        role_id -> Text,
    }
}

diesel::joinable!(job -> group (group_id));
diesel::joinable!(job -> job_type (job_type_id));
diesel::joinable!(job_runs -> job (job_id));
diesel::joinable!(user_group_mapping -> group (group_id));
diesel::joinable!(user_group_mapping -> role (role_id));
diesel::joinable!(user_group_mapping -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    group,
    job,
    job_runs,
    job_type,
    role,
    user,
    user_group_mapping,
);
