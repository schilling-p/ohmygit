// @generated automatically by Diesel CLI.

diesel::table! {
    branches (id) {
        id -> Uuid,
        creator_id -> Uuid,
        repository_id -> Uuid,
        name -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    comments (id) {
        id -> Uuid,
        creator_id -> Uuid,
        target_id -> Uuid,
        target_type -> Text,
        body -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    issues (id) {
        id -> Uuid,
        creator_id -> Uuid,
        repository_id -> Uuid,
        title -> Text,
        body -> Text,
        status -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    merge_requests (id) {
        id -> Uuid,
        creator_id -> Uuid,
        repository_id -> Uuid,
        source_branch_id -> Uuid,
        target_branch_id -> Uuid,
        title -> Text,
        description -> Text,
        status -> Text,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    organizations (id) {
        id -> Uuid,
        name -> Text,
        description -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    organizations_members (user_id, organization_id) {
        user_id -> Uuid,
        organization_id -> Uuid,
        role -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    repositories (id) {
        id -> Uuid,
        owner_id -> Nullable<Uuid>,
        owner_org_id -> Nullable<Uuid>,
        name -> Text,
        is_public -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_repository_roles (user_id, repository_id) {
        user_id -> Uuid,
        repository_id -> Uuid,
        role -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        hashed_pw -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(branches -> repositories (repository_id));
diesel::joinable!(branches -> users (creator_id));
diesel::joinable!(comments -> users (creator_id));
diesel::joinable!(issues -> repositories (repository_id));
diesel::joinable!(issues -> users (creator_id));
diesel::joinable!(merge_requests -> repositories (repository_id));
diesel::joinable!(merge_requests -> users (creator_id));
diesel::joinable!(organizations_members -> organizations (organization_id));
diesel::joinable!(organizations_members -> users (user_id));
diesel::joinable!(repositories -> organizations (owner_org_id));
diesel::joinable!(repositories -> users (owner_id));
diesel::joinable!(user_repository_roles -> repositories (repository_id));
diesel::joinable!(user_repository_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    branches,
    comments,
    issues,
    merge_requests,
    organizations,
    organizations_members,
    repositories,
    user_repository_roles,
    users,
);
