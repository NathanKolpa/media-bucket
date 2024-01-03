CREATE TABLE tag_implications
(
    from_id INTEGER NOT NULL,
    to_id  INTEGER NOT NULL,

    PRIMARY KEY (from_id, to_id),
    FOREIGN KEY (from_id) REFERENCES tags (tag_id),
    FOREIGN KEY (to_id) REFERENCES tags (tag_id),

    CONSTRAINT from_should_be_less_than_to CHECK (from_id < to_id)

);