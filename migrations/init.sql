CREATE extension if not exists "uuid-ossp";
CREATE table if not exists chats (
    id uuid primary key default uuid_generate_v4(),
    td_client_id bigint,
    td_chat_id bigint unique,
    is_read boolean
);

Create table if not exists messages (
    id uuid primary key default uuid_generate_v4(),
    chat_id uuid references chats,
    message_id bigint not null ,
    is_read boolean not null ,
    text text,
    created_at DATE,
    url text null
);

Create table if not exists matches (
    id uuid primary key default uuid_generate_v4(),
    url text unique,
    full_text text,
    chat_id int null
);

DO $$ BEGIN
    Create type ProfileReviewerStatus as ENUM ('PENDING', 'COMPLETED');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

Create table if not exists profile_reviewer (
    id uuid primary key default uuid_generate_v4(),
    chat_id bigint,
    status ProfileReviewerStatus
)