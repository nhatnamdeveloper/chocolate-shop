-- Add migration script here
CREATE TYPE product_type AS ENUM ('NORMAL', 'SPECIAL');

CREATE TABLE IF NOT EXISTS products (
    id serial primary key,
    name text,
    price integer,
    product_type product_type,
    images text[]
);