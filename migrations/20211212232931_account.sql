-- Add migration script here
CREATE TABLE IF NOT EXISTS "Account" (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    user_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT "PK_Account" PRIMARY KEY (id),

    CONSTRAINT "FK_Account_User" FOREIGN KEY (user_id)
        REFERENCES "User" (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);