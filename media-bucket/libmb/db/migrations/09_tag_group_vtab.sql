CREATE VIRTUAL TABLE tag_groups_vtab USING fts5
(
    group_id UNINDEXED,
    name,
    color UNINDEXED,
    created_at UNINDEXED,
    tokenize="trigram"
);

INSERT INTO tag_groups_vtab
SELECT *
from tag_group;

CREATE TRIGGER tag_groups_vtab_insert
    AFTER INSERT
    ON tag_group
BEGIN
    INSERT INTO tag_groups_vtab
    VALUES (new.group_id, new.name, new.color, new.created_at);
END;

CREATE TRIGGER tag_groups_vtab_delete
    AFTER DELETE
    ON tag_group
BEGIN
    DELETE FROM tag_groups_vtab WHERE group_id = old.group_id;
END;

CREATE TRIGGER tag_groups_vtab_update
    AFTER UPDATE
    ON tag_group
BEGIN
    DELETE FROM tag_groups_vtab WHERE group_id = old.group_id;

    INSERT INTO tag_groups_vtab
    VALUES (new.group_id, new.name, new.color, new.created_at);
END;