-- noinspection SqlNoDataSourceInspectionForFile

CREATE TABLE media_migrate
(
    media_id        INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    mime_type       TEXT    NOT NULL,
    mime_sub_type   TEXT    NOT NULL,
    sha256          TEXT    NOT NULL UNIQUE,
    md5             TEXT    NOT NULL,
    sha1            TEXT    NOT NULL,
    file_size       INTEGER NOT NULL,
    file_id         BLOB    NOT NULL UNIQUE,

    width           INTEGER NULL,
    height          INTEGER NULL,
    duration        INTEGER NULL,

    document_pages  INTEGER NULL,
    document_title  TEXT    NULL,
    document_author TEXT    NULL,
    page_width      INTEGER    NULL,
    page_height     INTEGER    NULL,

    CONSTRAINT width_height_not_null CHECK (NOT (mime_type IN ('image', 'video') AND (width IS NULL OR height IS NULL))),
    CONSTRAINT duration_not_null CHECK (NOT (mime_type IN ('video') AND (duration IS NULL)))
);

INSERT INTO media_migrate
(media_id, mime_type, mime_sub_type, sha256, md5, sha1, file_size, file_id, width, height, duration, document_pages,
 document_title, document_author, page_height, page_width)
SELECT media_id,
       mime_type,
       mime_sub_type,
       sha256,
       md5,
       sha1,
       file_size,
       file_id,
       width,
       height,
       duration,
       NULL,
       NULL,
       NULL,
       NULL,
       NULL
FROM media;

DROP TABLE media;
ALTER TABLE media_migrate
    RENAME TO media;