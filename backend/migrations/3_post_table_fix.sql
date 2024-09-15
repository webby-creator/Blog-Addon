DROP TABLE post;

CREATE TABLE post (
    id INTEGER NOT NULL,

    blog_id INTEGER NOT NULL REFERENCES blog(id) ON DELETE CASCADE,

    title TEXT NOT NULL,
    content TEXT NOT NULL,
    slug TEXT COLLATE NOCASE,

    status INTEGER NOT NULL,

    delete_reason TEXT,

    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    deleted_at DATETIME,

    UNIQUE(blog_id, slug),
    PRIMARY KEY ("id" AUTOINCREMENT)
);
