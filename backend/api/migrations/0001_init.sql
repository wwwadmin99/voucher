create extension if not exists pgcrypto;

create table users (
    id uuid primary key default gen_random_uuid(),
    name text not null
);

create table products (
    id uuid primary key default gen_random_uuid(),
    name text not null
);

create table voucher_balances (
    user_id uuid not null references users (id) on delete cascade,
    product_id uuid not null references products (id) on delete cascade,
    quantity integer not null default 0 check (quantity >= 0),
    primary key (user_id, product_id)
);

create table activations (
    id uuid primary key default gen_random_uuid(),
    user_id uuid not null references users (id) on delete cascade,
    product_id uuid not null references products (id) on delete cascade,
    activated_at timestamptz not null default now()
);

create index activations_user_id_activated_at_idx on activations (user_id, activated_at desc);
