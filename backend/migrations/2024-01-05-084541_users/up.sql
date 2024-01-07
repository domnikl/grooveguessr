CREATE TABLE users
(
    "id" UUID NOT NULL,
    "email" VARCHAR(70) NOT NULL,
    "name" VARCHAR(70) NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT "users_pkey" PRIMARY KEY ("id"),
    CONSTRAINT "users_email_unique" UNIQUE ("email")
);
