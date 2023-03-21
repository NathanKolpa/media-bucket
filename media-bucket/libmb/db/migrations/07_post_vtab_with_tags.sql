DROP TRIGGER posts_vtab_update;
DROP TRIGGER posts_vtab_insert;
DROP TRIGGER posts_vtab_delete;
DROP TRIGGER IF EXISTS posts_vtab_tags_posts_delete;
DROP TRIGGER IF EXISTS posts_vtab_tags_posts_insert;
DROP TRIGGER IF EXISTS posts_vtab_tags_posts_update;
DROP TRIGGER IF EXISTS posts_vtab_tags_update;
DROP TRIGGER IF EXISTS posts_vtab_items_delete;
DROP TRIGGER IF EXISTS posts_vtab_items_insert;
DROP TRIGGER IF EXISTS posts_vtab_items_update;
DROP TRIGGER IF EXISTS posts_vtab_media_update;
DROP TABLE posts_vtab;

CREATE VIRTUAL TABLE posts_vtab USING fts5
(
    post_id UNINDEXED,
    source,
    title,
    description,
    import_batch_id UNINDEXED,
    created_at,

    tags,

    original_name,
    original_directory,

    document_title,
    document_author,

    tokenize="trigram"
);

INSERT INTO posts_vtab
SELECT p.post_id, p.source, p.title, p.description, p.import_batch_id, p.created_at,
       (SELECT GROUP_CONCAT(t.name, ' ') FROM tags_posts tp INNER JOIN tags t ON t.tag_id = tp.tag_id WHERE tp.post_id = p.post_id),
       (SELECT GROUP_CONCAT(i.original_name, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_name IS NOT NULL),
       (SELECT GROUP_CONCAT(i.original_directory, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_directory IS NOT NULL),
       (SELECT GROUP_CONCAT(m.document_title, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_title IS NOT NULL),
       (SELECT GROUP_CONCAT(m.document_author, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_author IS NOT NULL)
from posts p;

-- POST TRIGGERS --

CREATE TRIGGER posts_vtab_insert
    AFTER INSERT
    ON posts
BEGIN
    INSERT INTO posts_vtab
    VALUES (new.post_id, new.source, new.title, new.description, new.import_batch_id, new.created_at,
                (SELECT GROUP_CONCAT(t.name, ' ') FROM tags_posts tp INNER JOIN tags t ON t.tag_id = tp.tag_id WHERE tp.post_id = p.post_id),
                (SELECT GROUP_CONCAT(i.original_name, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_name IS NOT NULL),
                (SELECT GROUP_CONCAT(i.original_directory, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_directory IS NOT NULL),
                (SELECT GROUP_CONCAT(m.document_title, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_title IS NOT NULL),
                (SELECT GROUP_CONCAT(m.document_author, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_author IS NOT NULL)
            );
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
    DELETE FROM posts_vtab WHERE post_id = new.post_id;

    INSERT INTO posts_vtab
    VALUES (new.post_id, new.source, new.title, new.description, new.import_batch_id, new.created_at,
            (SELECT GROUP_CONCAT(t.name, ' ') FROM tags_posts tp INNER JOIN tags t ON t.tag_id = tp.tag_id WHERE tp.post_id = p.post_id),
            (SELECT GROUP_CONCAT(i.original_name, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_name IS NOT NULL),
            (SELECT GROUP_CONCAT(i.original_directory, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_directory IS NOT NULL),
            (SELECT GROUP_CONCAT(m.document_title, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_title IS NOT NULL),
            (SELECT GROUP_CONCAT(m.document_author, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_author IS NOT NULL)
           );

    INSERT INTO posts_vtab
    VALUES (old.post_id, old.source, old.title, old.description, old.import_batch_id, old.created_at,
            (SELECT GROUP_CONCAT(t.name, ' ') FROM tags_posts tp INNER JOIN tags t ON t.tag_id = tp.tag_id WHERE tp.post_id = p.post_id),
            (SELECT GROUP_CONCAT(i.original_name, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_name IS NOT NULL),
            (SELECT GROUP_CONCAT(i.original_directory, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_directory IS NOT NULL),
            (SELECT GROUP_CONCAT(m.document_title, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_title IS NOT NULL),
            (SELECT GROUP_CONCAT(m.document_author, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_author IS NOT NULL)
           );
END;

-- TAGS TRIGGERS --

-- the only trigger for tags we have to implement

CREATE TRIGGER posts_vtab_tags_update
    AFTER UPDATE
    ON tags
BEGIN
    DELETE FROM posts_vtab WHERE post_id IN (select tp.post_id FROM tags_posts WHERE tp.tag_id = old.tag_id);

    INSERT INTO posts_vtab
    SELECT p.post_id, p.source, p.title, p.description, p.import_batch_id, p.created_at,
           (SELECT GROUP_CONCAT(t.name, ' ') FROM tags_posts tp INNER JOIN tags t ON t.tag_id = tp.tag_id WHERE tp.post_id = p.post_id),
           (SELECT GROUP_CONCAT(i.original_name, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_name IS NOT NULL),
           (SELECT GROUP_CONCAT(i.original_directory, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_directory IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_title, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_title IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_author, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_author IS NOT NULL)
    from posts p
    WHERE p.post_id IN (select tp.post_id FROM tags_posts WHERE tp.tag_id = old.tag_id);
END;

-- tags posts

CREATE TRIGGER posts_vtab_tags_posts_update
    AFTER UPDATE
    ON tags_posts
BEGIN
    DELETE FROM posts_vtab WHERE post_id IN (old.post_id, new.post_id);

    INSERT INTO posts_vtab
    SELECT p.post_id, p.source, p.title, p.description, p.import_batch_id, p.created_at,
           (SELECT GROUP_CONCAT(t.name, ' ') FROM tags_posts tp INNER JOIN tags t ON t.tag_id = tp.tag_id WHERE tp.post_id = p.post_id),
           (SELECT GROUP_CONCAT(i.original_name, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_name IS NOT NULL),
           (SELECT GROUP_CONCAT(i.original_directory, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_directory IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_title, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_title IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_author, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_author IS NOT NULL)
    from posts p
    WHERE p.post_id IN (old.post_id, new.post_id);
END;

CREATE TRIGGER posts_vtab_tags_posts_insert
    AFTER INSERT
    ON tags_posts
BEGIN
    DELETE FROM posts_vtab WHERE post_id = new.post_id;

    INSERT INTO posts_vtab
    SELECT p.post_id, p.source, p.title, p.description, p.import_batch_id, p.created_at,
           (SELECT GROUP_CONCAT(t.name, ' ') FROM tags_posts tp INNER JOIN tags t ON t.tag_id = tp.tag_id WHERE tp.post_id = p.post_id),
           (SELECT GROUP_CONCAT(i.original_name, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_name IS NOT NULL),
           (SELECT GROUP_CONCAT(i.original_directory, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_directory IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_title, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_title IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_author, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_author IS NOT NULL)
    from posts p
    WHERE p.post_id = new.post_id;
END;

CREATE TRIGGER posts_vtab_tags_posts_delete
    AFTER DELETE
    ON tags_posts
BEGIN
    DELETE FROM posts_vtab WHERE post_id = old.post_id;

    INSERT INTO posts_vtab
    SELECT p.post_id, p.source, p.title, p.description, p.import_batch_id, p.created_at,
           (SELECT GROUP_CONCAT(t.name, ' ') FROM tags_posts tp INNER JOIN tags t ON t.tag_id = tp.tag_id WHERE tp.post_id = p.post_id),
           (SELECT GROUP_CONCAT(i.original_name, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_name IS NOT NULL),
           (SELECT GROUP_CONCAT(i.original_directory, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_directory IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_title, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_title IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_author, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_author IS NOT NULL)
    from posts p
    WHERE p.post_id = old.post_id;
END;


-- ITEMS TRIGGERS --


CREATE TRIGGER posts_vtab_media_update
    AFTER UPDATE
    ON media
BEGIN
    DELETE FROM posts_vtab WHERE post_id IN (SELECT i.post_id FROM post_items i WHERE i.content_id IN(old.media_id, new.media_id));

    INSERT INTO posts_vtab
    SELECT p.post_id, p.source, p.title, p.description, p.import_batch_id, p.created_at,
           (SELECT GROUP_CONCAT(t.name, ' ') FROM tags_posts tp INNER JOIN tags t ON t.tag_id = tp.tag_id WHERE tp.post_id = p.post_id),
           (SELECT GROUP_CONCAT(i.original_name, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_name IS NOT NULL),
           (SELECT GROUP_CONCAT(i.original_directory, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_directory IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_title, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_title IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_author, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_author IS NOT NULL)
    from posts p
    WHERE p.post_id IN (SELECT i.post_id FROM post_items i WHERE i.content_id IN(old.media_id, new.media_id));
END;

CREATE TRIGGER posts_vtab_items_update
    AFTER UPDATE
    ON post_items
BEGIN
    DELETE FROM posts_vtab WHERE post_id IN (old.post_id, new.post_id);

    INSERT INTO posts_vtab
    SELECT p.post_id, p.source, p.title, p.description, p.import_batch_id, p.created_at,
           (SELECT GROUP_CONCAT(t.name, ' ') FROM tags_posts tp INNER JOIN tags t ON t.tag_id = tp.tag_id WHERE tp.post_id = p.post_id),
           (SELECT GROUP_CONCAT(i.original_name, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_name IS NOT NULL),
           (SELECT GROUP_CONCAT(i.original_directory, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_directory IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_title, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_title IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_author, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_author IS NOT NULL)
    from posts p
    WHERE p.post_id IN (old.post_id, new.post_id);
END;

CREATE TRIGGER posts_vtab_items_insert
    AFTER INSERT
    ON post_items
BEGIN
    DELETE FROM posts_vtab WHERE post_id = new.post_id;

    INSERT INTO posts_vtab
    SELECT p.post_id, p.source, p.title, p.description, p.import_batch_id, p.created_at,
           (SELECT GROUP_CONCAT(t.name, ' ') FROM tags_posts tp INNER JOIN tags t ON t.tag_id = tp.tag_id WHERE tp.post_id = p.post_id),
           (SELECT GROUP_CONCAT(i.original_name, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_name IS NOT NULL),
           (SELECT GROUP_CONCAT(i.original_directory, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_directory IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_title, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_title IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_author, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_author IS NOT NULL)
    from posts p
    WHERE p.post_id = new.post_id;
END;

CREATE TRIGGER posts_vtab_items_delete
    AFTER DELETE
    ON post_items
BEGIN
    DELETE FROM posts_vtab WHERE post_id = old.post_id;

    INSERT INTO posts_vtab
    SELECT p.post_id, p.source, p.title, p.description, p.import_batch_id, p.created_at,
           (SELECT GROUP_CONCAT(t.name, ' ') FROM tags_posts tp INNER JOIN tags t ON t.tag_id = tp.tag_id WHERE tp.post_id = p.post_id),
           (SELECT GROUP_CONCAT(i.original_name, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_name IS NOT NULL),
           (SELECT GROUP_CONCAT(i.original_directory, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_directory IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_title, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_title IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_author, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_author IS NOT NULL)
    from posts p
    WHERE p.post_id = old.post_id;
END;