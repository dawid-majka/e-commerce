-- Add migration script here

CREATE TABLE
    stores(
        id uuid NOT NULL,
        PRIMARY KEY (id),
        name TEXT NOT NULL,
        userId uuid NOT NULL,
        createdAt timestamptz NOT NULL,
        updatedAt timestamptz NOT NULL
    );