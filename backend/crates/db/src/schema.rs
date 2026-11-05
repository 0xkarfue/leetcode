// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int8,
        #[max_length = 255]
        google_sub -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        email_verified -> Bool,
        #[max_length = 32]
        username -> Nullable<Varchar>,
        #[max_length = 100]
        display_name -> Varchar,
        avatar_url -> Nullable<Text>,
        #[max_length = 16]
        role -> Varchar,
        is_active -> Bool,
        onboarding_completed -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_login_at -> Nullable<Timestamptz>,
    }
}
