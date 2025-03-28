CREATE TABLE blocks (
    id UUID PRIMARY KEY,
    hash VARCHAR(64) UNIQUE NOT NULL,
    previous_hash VARCHAR(64) NOT NULL,
    transactions TEXT NOT NULL,
    nonce BIGINT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);
