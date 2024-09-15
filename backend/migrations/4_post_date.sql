ALTER TABLE post ADD COLUMN post_date DATETIME;
UPDATE post SET post_date = created_at;