create table if not exists account (
    id int generated always as identity,
    name text,
    email text,
    password text,
    primary key (id)
);

create table if not exists feed (
    id int generated always as identity,
    url text,
    account_id int,
    primary key (id),

    constraint fk_feed_belongs_to_account
        foreign key (account_id)
        references account (id)
        on delete cascade
);
