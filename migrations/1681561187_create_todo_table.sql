create table if not exists todos (
    id bigserial primary key,
    name varchar(255) not null,
    description text not null default '',
    completed boolean not null default false,
    created_at timestamp not null default current_timestamp,
    updated_at timestamp not null default current_timestamp
);