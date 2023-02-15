-- noinspection SqlNoDataSourceInspectionForFile

CREATE TABLE IF NOT EXISTS import_batches
(
    import_batch_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT
);

CREATE TABLE IF NOT EXISTS  media
(
    media_id      INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    width         INTEGER NULL,
    height        INTEGER NULL,
    duration      INTEGER NULL,
    mime_type     TEXT    NOT NULL,
    mime_sub_type TEXT    NOT NULL,
    sha256        TEXT    NOT NULL UNIQUE,
    md5           TEXT    NOT NULL,
    sha1          TEXT    NOT NULL,
    file_size     INTEGER NOT NULL,
    file_id       BLOB    NOT NULL UNIQUE,

    CONSTRAINT allowed_mime_types CHECK (mime_type IN ('image', 'video')),
    CONSTRAINT width_height_not_null CHECK (NOT (mime_type IN ('image', 'video') AND (width IS NULL OR height IS NULL))),
    CONSTRAINT duration_not_null CHECK (NOT (mime_type IN ('video') AND (duration IS NULL)))
);

CREATE TABLE IF NOT EXISTS  content
(
    content_id   INTEGER NOT NULL PRIMARY KEY,
    thumbnail_id INTEGER NOT NULL,

    FOREIGN KEY (content_id) REFERENCES media (media_id),
    FOREIGN KEY (thumbnail_id) REFERENCES media (media_id)
);

CREATE TABLE IF NOT EXISTS  uploads
(
    upload_id          INTEGER                            NOT NULL PRIMARY KEY AUTOINCREMENT,
    original_name      TEXT                               NULL,
    original_accessed  DATETIME                           NULL,
    original_modified  DATETIME                           NULL,
    original_directory TEXT                               NULL,
    uploaded_at        DATETIME DEFAULT CURRENT_TIMESTAMP NULL,
    content_id         INTEGER                            NOT NULL,

    FOREIGN KEY (content_id) REFERENCES content (content_id)
);

CREATE TABLE IF NOT EXISTS  posts
(
    post_id         INTEGER                            NOT NULL PRIMARY KEY AUTOINCREMENT,
    source          TEXT                               NULL,
    title           TEXT                               NULL,
    description     TEXT                               NULL,
    import_batch_id INTEGER                            NOT NULL,
    created_at      DATETIME DEFAULT CURRENT_TIMESTAMP NULL,

    FOREIGN KEY (import_batch_id) REFERENCES import_batches (import_batch_id)
);

CREATE TABLE IF NOT EXISTS  post_items
(
    post_id    INTEGER NOT NULL,
    item_order INTEGER NOT NULL,
    content_id INTEGER NOT NULL,

    PRIMARY KEY (post_id, item_order),
    FOREIGN KEY (post_id) REFERENCES posts (post_id),
    FOREIGN KEY (content_id) REFERENCES content (content_id)
);

CREATE TABLE IF NOT EXISTS  tag_group
(
    group_id   INTEGER                            NOT NULL PRIMARY KEY AUTOINCREMENT,
    name       TEXT                               NOT NULL UNIQUE,
    color      TEXT                               NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NULL
);

CREATE TABLE IF NOT EXISTS  tags
(
    tag_id     INTEGER                            NOT NULL PRIMARY KEY AUTOINCREMENT,
    name       TEXT                               NOT NULL UNIQUE,
    group_id   INTEGER                            NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NULL,

    FOREIGN KEY (group_id) REFERENCES tag_group (group_id)
);

CREATE TABLE IF NOT EXISTS tags_posts
(
    post_id INTEGER NOT NULL,
    tag_id  INTEGER NOT NULL,

    PRIMARY KEY (post_id, tag_id),
    FOREIGN KEY (post_id) REFERENCES posts (post_id),
    FOREIGN KEY (tag_id) REFERENCES tags (tag_id)
);
