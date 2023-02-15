ALTER TABLE media
ADD COLUMN video_encoding TEXT NULL;

UPDATE media SET video_encoding = 'h264' WHERE mime_type = 'video';
