CREATE TABLE accounts
(
    id       SERIAL PRIMARY KEY,
    username VARCHAR(64)  NOT NULL,
    password VARCHAR(256) NOT NULL
)
