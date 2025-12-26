-- Add up migration script here
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

CREATE TYPE action_log_type AS ENUM (
    'FileUpload',
    'FileDeletion',
    'FolderCreation',
    'FolderDeletion',
    'UserCreation',
    'AccountCompletion'
);

CREATE TYPE access_type AS ENUM (
    'ReadOnly',
    'Edit'
);

CREATE TYPE upload_status AS ENUM (
    'Pending',
    'Completed',
    'Failed'
);

CREATE TABLE users
(
    id              UUID PRIMARY KEY,
    email           TEXT UNIQUE NOT NULL,
    full_name       TEXT UNIQUE NOT NULL,
    password_hash   TEXT        NOT NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    role            user_role   NOT NULL,
    allowed_storage BIGINT      NOT NULL,
    taken_storage   BIGINT      NOT NULL
);

CREATE TABLE white_listed_users
(
    id         UUID PRIMARY KEY,
    full_name  TEXT        NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    email      TEXT UNIQUE NOT NULL
);

CREATE TABLE folders
(
    id               UUID PRIMARY KEY,
    name             TEXT        NOT NULL,
    owner_id         UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    parent_folder_id UUID        NOT NULL REFERENCES folders (id) ON DELETE CASCADE
);


CREATE TABLE files
(
    id               UUID PRIMARY KEY,
    name             TEXT          NOT NULL,
    owner_id         UUID          NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    file_type        file_type     NOT NULL,
    parent_folder_id UUID          NOT NULL REFERENCES folders (id) ON DELETE CASCADE,
    is_deleted       BOOLEAN       NOT NULL DEFAULT FALSE,
    ttl              TIMESTAMPTZ,
    size             BIGINT        NOT NULL,
    upload_status    upload_status NOT NULL
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

CREATE TABLE shared_file
(
    id          UUID PRIMARY KEY,
    user_id     UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    owner_id    UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    file_id     UUID        NOT NULL REFERENCES files (id) ON DELETE CASCADE,
    access_type access_type NOT NULL
);

CREATE TABLE global_files
(
    id          UUID PRIMARY KEY,
    original_id UUID NOT NULL REFERENCES files (id) ON DELETE CASCADE
);

CREATE TABLE labels
(
    id       UUID PRIMARY KEY,
    name     TEXT NOT NULL,
    color    TEXT NOT NULL,
    owner_id UUID NOT NULL REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE file_labels
(
    id       UUID PRIMARY KEY,
    file_id  UUID REFERENCES files (id) ON DELETE CASCADE,
    label_id UUID REFERENCES labels (id) ON DELETE CASCADE
);