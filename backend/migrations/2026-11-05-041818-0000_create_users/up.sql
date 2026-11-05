-- Your SQL goes here

CREATE TABLE users (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    google_sub VARCHAR(255) NOT NULL UNIQUE,

    email VARCHAR(255) NOT NULL,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,

    username VARCHAR(32) UNIQUE,
    display_name VARCHAR(100) NOT NULL,
    avatar_url TEXT,

    role VARCHAR(16) NOT NULL DEFAULT 'user'
        CHECK (role IN ('user', 'admin')),

    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    onboarding_completed BOOLEAN NOT NULL DEFAULT FALSE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX users_email_unique
ON users (LOWER(email));