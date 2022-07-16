CREATE TABLE IF NOT EXISTS account (
    id INT GENERATED ALWAYS AS IDENTITY,
    name TEXT,
    email TEXT,
    password TEXT,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS feed (
    id INT GENERATED ALWAYS AS IDENTITY,
    url TEXT,
    account_id INT,
    PRIMARY KEY (id),

    CONSTRAINT fk_feed_belongs_to_account
        FOREIGN KEY (account_id)
        REFERENCES account (id)
        ON DELETE CASCADE
);
