CREATE EXTENSION if not exists vector;
CREATE OR REPLACE FUNCTION update_timestamp()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TABLE if not exists image_embeddings
(
    id          serial4                     NOT NULL,
    image_path  text                        NOT NULL,
    description text                        NOT NULL,
    embedding   vector                      NULL,
    created_at  TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp,
    updated_at  TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp,
    CONSTRAINT image_embeddings_pkey PRIMARY KEY (id)
);

CREATE OR REPLACE TRIGGER set_timestamp
    BEFORE UPDATE
    ON image_embeddings
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp();
