CREATE TABLE account (
    id INT GENERATED ALWAYS AS IDENTITY,
    name TEXT,
    email TEXT,
    password TEXT
);

CREATE TABLE feed (
    id INT GENERATED ALWAYS AS IDENTITY,
    url TEXT,
    account_id INT,

    CONSTRAINT feed_belongs_to_account
        FOREIGN KEY (account_id)
        REFERENCES account (id)
        ON DELETE CASCADE
);
