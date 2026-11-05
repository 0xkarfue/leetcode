
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::schema::users;

// Represents an existing user in PostgreSQL.
#[derive(Debug, Queryable, Selectable, Identifiable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i64,

    pub google_sub: String,

    pub email: String,
    pub email_verified: bool,

    pub username: Option<String>,
    pub display_name: String,
    pub avatar_url: Option<String>,

    pub role: String,
    pub is_active: bool,
    pub onboarding_completed: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

// Represents a new user being inserted into PostgreSQL.
#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub google_sub: &'a str,

    pub email: &'a str,
    pub email_verified: bool,

    pub display_name: &'a str,
    pub avatar_url: Option<&'a str>,
}

// Represents fields that can be updated on an existing user.
#[derive(Debug, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser<'a> {
    pub username: Option<&'a str>,
    pub display_name: Option<&'a str>,
    pub avatar_url: Option<&'a str>,
    pub onboarding_completed: Option<bool>,
}
