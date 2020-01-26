CREATE TABLE releases
(
    id            SERIAL PRIMARY KEY,
    version_major INTEGER   NOT NULL,
    version_minor INTEGER   NOT NULL,
    version_patch INTEGER   NOT NULL,
    creation_date TIMESTAMP NOT NULL,
    active        BOOLEAN   NOT NULL DEFAULT false,
    description   TEXT,
    changelog     TEXT,
    CONSTRAINT unique_version UNIQUE (version_major, version_minor, version_patch)
)
