-- add all meta data fields from uploads to post_items
ALTER TABLE post_items
    ADD original_name TEXT NULL;
ALTER TABLE post_items
    ADD original_accessed DATETIME NULL;
ALTER TABLE post_items
    ADD original_modified DATETIME NULL;
ALTER TABLE post_items
    ADD original_directory TEXT NULL;
ALTER TABLE post_items
    ADD uploaded_at DATETIME DEFAULT NULL NULL;

-- move upload data to post items
INSERT
OR REPLACE INTO post_items(post_id, item_order, content_id, original_name, original_accessed, original_modified, original_directory, uploaded_at)
SELECT pi.post_id,
       pi.item_order,
       pi.content_id,
       u.original_name, u.original_accessed,
       u.original_modified,
       u.original_directory,
       u.uploaded_at
FROM post_items pi
         LEFT JOIN posts p ON pi.post_id = p.post_id
         LEFT JOIN uploads u ON pi.content_id = u.content_id AND ABS(julianday(u.uploaded_at) - julianday(p.created_at)) = (SELECT min(ABS(julianday(uploaded_at) - julianday(p.created_at))) as diff FROM uploads WHERE pi.content_id = content_id);

-- drop old
DROP TABLE uploads