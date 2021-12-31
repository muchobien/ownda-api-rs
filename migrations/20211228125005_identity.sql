-- Add migration script here
CREATE TYPE "provider_enum" AS ENUM ('GOOGLE', 'APPLE', 'LOCAL');

CREATE TABLE IF NOT EXISTS "Identity" (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL,
    provider "provider_enum" NOT NULL,
    hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT "PK_Identity" PRIMARY KEY (id),

    CONSTRAINT "FK_Identity_User" FOREIGN KEY (user_id)
        REFERENCES "User" (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);