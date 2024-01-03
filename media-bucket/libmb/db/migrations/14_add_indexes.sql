CREATE INDEX posts_created_at_index ON posts(created_at);
CREATE INDEX tags_created_at_index ON tags(created_at);
CREATE INDEX tag_group_created_at_index ON tag_group(created_at);


CREATE INDEX media_mime_type_index ON media(mime_type);
CREATE INDEX media_mime_sub_type_index ON media(mime_sub_type);
CREATE INDEX media_duration_index ON media(duration);


