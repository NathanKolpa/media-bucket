CREATE VIRTUAL TABLE posts_vtab USING fts5
(
    post_id UNINDEXED,
    source,
    title,
    description,
    import_batch_id UNINDEXED,
    created_at,
    tokenize="trigram"
);

INSERT INTO posts_vtab
SELECT *
from posts;

CREATE TRIGGER posts_vtab_insert
    AFTER INSERT
    ON posts
BEGIN
    INSERT INTO posts_vtab
    VALUES (new.post_id, new.source, new.title, new.description, new.import_batch_id, new.created_at);
END;

CREATE TRIGGER posts_vtab_delete
    AFTER DELETE
    ON posts
BEGIN
    DELETE FROM posts_vtab WHERE post_id = old.post_id;
END;

CREATE TRIGGER posts_vtab_update
    AFTER UPDATE
    ON tags
BEGIN
    DELETE FROM posts_vtab WHERE post_id = old.post_id;

    INSERT INTO posts_vtab
    VALUES (new.post_id, new.source, new.title, new.description, new.import_batch_id, new.created_at);
END;