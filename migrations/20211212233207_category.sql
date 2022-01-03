-- Add migration script here
CREATE TABLE IF NOT EXISTS "Category" (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    parent_id UUID DEFAULT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT "PK_Category" PRIMARY KEY (id),
    CONSTRAINT "fk_category_parent_id" FOREIGN KEY (parent_id) REFERENCES "Category" (id) ON DELETE
    SET NULL
);