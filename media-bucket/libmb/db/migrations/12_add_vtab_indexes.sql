DROP TABLE posts_vtab;

CREATE VIRTUAL TABLE posts_vtab USING fts5
(
    post_id,
    source,
    title,
    description,
    import_batch_id,
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


DROP TABLE tags_vtab;
CREATE VIRTUAL TABLE tags_vtab USING fts5
(
    tag_id,
    name,
    group_id,
    created_at,
    tokenize="trigram"
);

INSERT INTO tags_vtab
SELECT *
from tags;



DROP TABLE tag_groups_vtab;
CREATE VIRTUAL TABLE tag_groups_vtab USING fts5
(
    group_id,
    name,
    color,
    created_at,
    tokenize="trigram"
);

INSERT INTO tag_groups_vtab
SELECT *
from tag_group;