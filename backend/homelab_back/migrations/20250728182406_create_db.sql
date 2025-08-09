CREATE TYPE file_type AS ENUM (
    'Text',
    'Image',
    'Video',
    'Unknown'
    );

CREATE TYPE user_role AS ENUM (
    'user',
    'admin'
    );

CREATE TABLE users
(
    id            UUID PRIMARY KEY,
    email         TEXT UNIQUE NOT NULL,
    password_hash TEXT        NOT NULL,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    role          user_role   NOT NULL
);

CREATE TABLE folders
(
    id               UUID PRIMARY KEY,
    name             TEXT        NOT NULL,
    owner_id         UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    parent_folder_id UUID REFERENCES folders (id) ON DELETE CASCADE
);


CREATE TABLE files
(
    id        UUID PRIMARY KEY,
    name      TEXT      NOT NULL,
    owner_id  UUID      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    file_type file_type NOT NULL,
    parent_folder_id UUID REFERENCES folders (id) ON DELETE CASCADE
);

CREATE TYPE action_log_type AS ENUM (
    'FileUpload',
    'FileDeletion',
    'FolderCreation',
    'FolderDeletion',
    'UserCreation',
    'AccountCompletion'
    );

CREATE TABLE action_logs
(
    id         UUID PRIMARY KEY,
    user_id    UUID REFERENCES users (id) ON DELETE CASCADE,
    log_type   action_log_type NOT NULL,
    file_id    UUID            REFERENCES files (id) ON DELETE SET NULL,
    folder_id  UUID            REFERENCES folders (id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ     NOT NULL DEFAULT NOW()
);
