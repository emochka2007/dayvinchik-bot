CREATE OR REPLACE FUNCTION update_timestamp()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE extension if not exists "uuid-ossp";
CREATE table if not exists chats
(
    id                   uuid primary key                     default uuid_generate_v4(),
    chat_id              bigint unique,
    -- mb unique --
    last_read_message_id bigint,
    last_message_id      bigint,
    title                text,
    created_at           TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp,
    updated_at           TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp
);
CREATE OR REPLACE TRIGGER set_timestamp
    BEFORE UPDATE
    ON chats
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

Create table if not exists messages
(
    id         uuid primary key                     default uuid_generate_v4(),
    chat_id    bigint,
    message_id bigint                      not null,
    is_read    boolean                     not null,
    text       text,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp,
    url        text                        null
);
CREATE OR REPLACE TRIGGER set_timestamp
    BEFORE UPDATE
    ON messages
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp();
Create table if not exists matches
(
    id         uuid primary key                     default uuid_generate_v4(),
    url        text unique,
    full_text  text,
    chat_id    int                         null,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp
);

CREATE OR REPLACE TRIGGER set_timestamp
    BEFORE UPDATE
    ON matches
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

Create table if not exists profile_reviewers
(
    id         uuid primary key                     default uuid_generate_v4(),
    chat_id    bigint,
    text       text,
    file_ids   int[],
    -- PENDING, WAITING, COMPLETED
    status     text                                 default 'WAITING',
    score      int                         null,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp
);
CREATE OR REPLACE TRIGGER set_timestamp
    BEFORE UPDATE
    ON profile_reviewers
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

Create table if not exists tasks
(
    id         uuid primary key                     default uuid_generate_v4(),
    message    text,
    -- WAITING, COMPLETED
    status     text                                 DEFAULT 'WAITING',
    request    text,
    response   text,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp
);
CREATE OR REPLACE TRIGGER set_timestamp
    BEFORE UPDATE
    ON tasks
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp();
