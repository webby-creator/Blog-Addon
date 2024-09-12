CREATE TABLE blog (
    id INTEGER NOT NULL,

    external_website_id TEXT,
    external_member_id TEXT,

    name TEXT NOT NULL,

    setup_position INTEGER NOT NULL,

    delete_reason TEXT,

    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    deleted_at DATETIME,

    UNIQUE(external_website_id, external_member_id, deleted_at),
    PRIMARY KEY ("id" AUTOINCREMENT)
);

CREATE TABLE post (
    id INTEGER NOT NULL,

    blog_id INTEGER NOT NULL REFERENCES blog(id) ON DELETE CASCADE,

    title TEXT NOT NULL,
    content TEXT NOT NULL,
    slug TEXT COLLATE NOCASE NOT NULL,

    status INTEGER NOT NULL,

    delete_reason TEXT,

    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    deleted_at DATETIME,

    UNIQUE(blog_id, slug),
    PRIMARY KEY ("id" AUTOINCREMENT)
);

CREATE TABLE author (
    id INTEGER NOT NULL,

    blog_id INTEGER NOT NULL REFERENCES blog(id) ON DELETE CASCADE,

    external_member_id TEXT NOT NULL UNIQUE,

    name TEXT NOT NULL,
    email TEXT,

    PRIMARY KEY ("id" AUTOINCREMENT)
);

CREATE TABLE comment (
    id INTEGER NOT NULL,

    blog_id INTEGER NOT NULL REFERENCES blog(id) ON DELETE CASCADE,
    post_id INTEGER NOT NULL REFERENCES post(id) ON DELETE CASCADE,

    external_member_id TEXT,

    author_name TEXT NOT NULL,

    email TEXT UNIQUE,
    comment TEXT NOT NULL,

    status INTEGER NOT NULL,

    delete_reason TEXT,
    deleted_at DATETIME,

    PRIMARY KEY ("id" AUTOINCREMENT)
);

CREATE TABLE category (
    id INTEGER NOT NULL,

    name TEXT NOT NULL UNIQUE,

    PRIMARY KEY ("id" AUTOINCREMENT)
);

CREATE TABLE tag (
    id INTEGER NOT NULL,

    name TEXT NOT NULL UNIQUE,

    PRIMARY KEY ("id" AUTOINCREMENT)
);

CREATE TABLE post_category
(
    blog_id INTEGER NOT NULL REFERENCES blog(id) ON DELETE CASCADE,

    post_id INTEGER NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    category_id INTEGER NOT NULL REFERENCES category(id) ON DELETE CASCADE,

    PRIMARY KEY (blog_id, post_id, category_id)
);

CREATE TABLE post_tag
(
    blog_id INTEGER NOT NULL REFERENCES blog(id) ON DELETE CASCADE,

    post_id INTEGER NOT NULL REFERENCES post(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tag(id) ON DELETE CASCADE,

    PRIMARY KEY (blog_id, post_id, tag_id)
);