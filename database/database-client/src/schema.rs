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
    job_details_http (id) {
        id -> Text,
        url -> Text,
    }
}

diesel::table! {
    job_details_ping (id) {
        id -> Text,
        host -> Text,
    }
}

diesel::table! {
    job_result_http (id) {
        id -> Text,
        ip_address -> Inet,
        status_code -> Int4,
        latency -> Int4,
        payload -> Bytea,
    }
}

diesel::table! {
    job_result_ping (id) {
        id -> Text,
        ip_address -> Inet,
        latency -> Int4,
    }
}

diesel::table! {
    job_runs (id) {
        id -> Text,
        job_id -> Text,
        timestamp -> Timestamp,
        triggered_notification -> Bool,
        batch_id -> Text,
        reachable -> Bool,
    }
}

diesel::table! {
    job_type (id) {
        id -> Text,
        name -> Nullable<Text>,
    }
}

diesel::table! {
    refresh_token (jti) {
        jti -> Text,
        user_id -> Text,
        ip_address -> Text,
        user_agent -> Text,
        expires_on -> Timestamp,
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
diesel::joinable!(job_details_http -> job (id));
diesel::joinable!(job_details_ping -> job (id));
diesel::joinable!(job_result_http -> job_runs (id));
diesel::joinable!(job_result_ping -> job_runs (id));
diesel::joinable!(job_runs -> job (job_id));
diesel::joinable!(refresh_token -> user (user_id));
diesel::joinable!(user_group_mapping -> group (group_id));
diesel::joinable!(user_group_mapping -> role (role_id));
diesel::joinable!(user_group_mapping -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    group,
    job,
    job_details_http,
    job_details_ping,
    job_result_http,
    job_result_ping,
    job_runs,
    job_type,
    refresh_token,
    role,
    user,
    user_group_mapping,
);
