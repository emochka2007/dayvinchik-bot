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
    chat_id              bigint unique               not null,
    -- mb unique --
    last_read_message_id bigint                      not null,
    last_message_id      bigint                      not null,
    title                text                        not null,
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
    chat_id    bigint                      not null,
    message_id bigint                      not null,
    is_read    boolean                     not null,
    text       text                        null,
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
    url        text unique                 not null,
    full_text  text                        null,
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
    id             uuid primary key                     default uuid_generate_v4(),
    chat_id        bigint                      not null,
    text           text                        null,
    file_ids       int[]                                default array []::int[],
    local_img_path text                        not null,
    -- PENDING, WAITING, COMPLETE
    status         text                                 default 'WAITING',
    score          int                                  default 0,
    created_at     TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp,
    updated_at     TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp
);
CREATE OR REPLACE TRIGGER set_timestamp
    BEFORE UPDATE
    ON profile_reviewers
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

Create table if not exists tasks
(
    id         uuid primary key                     default uuid_generate_v4(),
    message    text                        not null,
    -- WAITING, COMPLETE
    status     text                        not null,
    request    text                        not null,
    response   text                        not null,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp
);
CREATE OR REPLACE TRIGGER set_timestamp
    BEFORE UPDATE
    ON tasks
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

Create table if not exists superlikes
(
    id                  uuid primary key                     default uuid_generate_v4(),
    message             text                        not null,
    -- WAITING, COMPLETE
    profile_reviewer_id uuid references profile_reviewers,
    status              text                                 DEFAULT 'WAITING',
    created_at          TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp,
    updated_at          TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT current_timestamp
)
