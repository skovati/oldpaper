create table if not exists "user" (
    id int generated always as identity,
    name text,
    email text,
    password text,
    primary key (id)
);

create table if not exists feed (
    id int generated always as identity,
    user_id int,
    url text,
    primary key (id),

    constraint fk_feed_belongs_to_account
        foreign key (user_id)
        references "user" (id)
        on delete cascade
);
