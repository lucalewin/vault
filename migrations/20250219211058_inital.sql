create table if not exists "users" (
    "id" serial primary key,

    -- standard user information
    "username" varchar(255) not null unique,
    "email" varchar(255) not null unique,
    "password_hash" varchar(255) not null,

    -- salt used to derive the master key and recovery key
    "master_salt" bytea not null,
    "recovery_salt" bytea not null,

    -- salt used to hash the recovery phrase
    "recovery_phrase_hash" varchar(255) not null,

    -- encrypted master key and the nonce used to encrypt it (joined together)
    "recovery_code" bytea not null
);

-- create table if not exists "passwords" (
--     "id" serial primary key,
--     "user_id" integer not null references "users"("id"),
--     "url" varchar(255),
--     "username" varchar(255),
--     -- the encrypted password and the nonce used to encrypt it (joined together)
--     "password" varchar(255) not null,
--     "created_at" timestamp not null default current_timestamp
-- );

create table if not exists "credentials" (
    "id" serial primary key,
    "user_id" integer not null references "users"("id"),
    "service" varchar(255) not null,
    "username" varchar(255) not null,
    "password" bytea not null, -- encrypted password
    "created_at" timestamp not null default current_timestamp
);
