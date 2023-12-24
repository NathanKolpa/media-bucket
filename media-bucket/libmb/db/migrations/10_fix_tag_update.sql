DROP TRIGGER IF EXISTS posts_vtab_tags_update;

CREATE TRIGGER posts_vtab_tags_update
    AFTER UPDATE
    ON tags
BEGIN
    DELETE FROM posts_vtab WHERE post_id IN (select tp.post_id FROM tags_posts tp WHERE tp.tag_id = old.tag_id);

    INSERT INTO posts_vtab
    SELECT p.post_id, p.source, p.title, p.description, p.import_batch_id, p.created_at,
           (SELECT GROUP_CONCAT(t.name, ' ') FROM tags_posts tp INNER JOIN tags t ON t.tag_id = tp.tag_id WHERE tp.post_id = p.post_id),
           (SELECT GROUP_CONCAT(i.original_name, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_name IS NOT NULL),
           (SELECT GROUP_CONCAT(i.original_directory, ' ') FROM post_items i WHERE i.post_id = p.post_id AND i.original_directory IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_title, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_title IS NOT NULL),
           (SELECT GROUP_CONCAT(m.document_author, ' ') FROM post_items i INNER JOIN media m ON m.media_id = i.content_id WHERE i.post_id = p.post_id AND m.document_author IS NOT NULL)
    from posts p
    WHERE p.post_id IN (select tp.post_id FROM tags_posts tp WHERE tp.tag_id = old.tag_id);
END;