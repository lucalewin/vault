-- Add migration script here
create table if not exists "authenticator_codes" (
    "id" SERIAL PRIMARY KEY,
    "user_id" integer not null references "users"("id"),
    "username" VARCHAR(255) NOT NULL,
    "service_name" VARCHAR(255) NOT NULL,
    "secret_key" TEXT NOT NULL,
    "created_at" TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
