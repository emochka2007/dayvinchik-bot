CREATE EXTENSION vector;
CREATE TABLE items
(
    id        bigserial PRIMARY KEY,
    embedding vector(3)
);