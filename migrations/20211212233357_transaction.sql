-- Add migration script here
CREATE TYPE "transaction_type_enum" AS ENUM ('INCOME', 'EXPENSE');

CREATE TABLE IF NOT EXISTS "Transaction" (
    id UUID NOT NULL DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    amount DECIMAL(10, 2) NOT NULL,
    account_id UUID NOT NULL,
    category_id UUID NOT NULL,
    type "transaction_type_enum" NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT "PK_Transaction" PRIMARY KEY (id),

    CONSTRAINT "FK_Transaction_Account" FOREIGN KEY (account_id)
        REFERENCES "Account" (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,

    CONSTRAINT "FK_Transaction_Category" FOREIGN KEY (category_id)
        REFERENCES "Category" (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);