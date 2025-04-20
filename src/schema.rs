// @generated automatically by Diesel CLI.

diesel::table! {
    collaborators (id) {
        id -> Int4,
        user_id -> Int4,
        project_id -> Int4,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        author -> Int4,
        comment -> Text,
        created_at -> Timestamp,
        contract_id -> Nullable<Int4>,
    }
}

diesel::table! {
    contracts (id) {
        id -> Int4,
        project_id -> Int4,
        author_id -> Int4,
        #[max_length = 128]
        grpc_method -> Varchar,
        #[max_length = 128]
        tag -> Nullable<Varchar>,
        #[max_length = 128]
        errors_response -> Nullable<Varchar>,
        #[max_length = 256]
        path -> Nullable<Varchar>,
        #[max_length = 128]
        query -> Nullable<Varchar>,
        #[max_length = 256]
        body -> Nullable<Varchar>,
        response -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 64]
        http_method -> Nullable<Varchar>,
        #[max_length = 256]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        #[max_length = 255]
        project_link -> Nullable<Varchar>,
        #[max_length = 500]
        description -> Nullable<Varchar>,
        updated_at -> Timestamp,
        #[max_length = 255]
        proto_file -> Nullable<Varchar>,
        creator_id -> Int4,
        collaborators -> Int4,
    }
}

diesel::table! {
    sessions (id) {
        id -> Uuid,
        user_id -> Int4,
        #[max_length = 512]
        token -> Varchar,
        #[max_length = 45]
        ip_address -> Varchar,
        user_agent -> Nullable<Text>,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        is_active -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 128]
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 128]
        password_hash -> Nullable<Varchar>,
        is_active -> Bool,
        #[max_length = 128]
        photo_url -> Nullable<Varchar>,
    }
}

diesel::joinable!(collaborators -> users (user_id));
diesel::joinable!(comments -> contracts (contract_id));
diesel::joinable!(comments -> users (author));
diesel::joinable!(contracts -> projects (project_id));
diesel::joinable!(contracts -> users (author_id));
diesel::joinable!(projects -> users (creator_id));
diesel::joinable!(sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    collaborators,
    comments,
    contracts,
    projects,
    sessions,
    users,
);
