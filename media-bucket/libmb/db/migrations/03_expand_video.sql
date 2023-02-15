ALTER TABLE content
ADD COLUMN compatibility_content_id INTEGER NULL REFERENCES media(media_id);