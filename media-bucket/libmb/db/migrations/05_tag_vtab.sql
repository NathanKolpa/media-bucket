CREATE VIRTUAL TABLE tags_vtab USING fts5
(
    tag_id UNINDEXED,
    name,
    group_id UNINDEXED,
    created_at UNINDEXED,
    tokenize="trigram"
);

INSERT INTO tags_vtab
SELECT *
from tags;

CREATE TRIGGER tags_vtab_insert
    AFTER INSERT
    ON tags
BEGIN
    INSERT INTO tags_vtab
    VALUES (new.tag_id, new.name, new.group_id, new.created_at);
END;

CREATE TRIGGER tags_vtab_delete
    AFTER DELETE
    ON tags
BEGIN
    DELETE FROM tags_vtab WHERE tag_id = old.tag_id;
END;

CREATE TRIGGER tags_vtab_update
    AFTER UPDATE
    ON tags
BEGIN
    DELETE FROM tags_vtab WHERE tag_id = old.tag_id;

    INSERT INTO tags_vtab
    VALUES (new.tag_id, new.name, new.group_id, new.created_at);
END;