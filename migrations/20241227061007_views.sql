CREATE TABLE IF NOT EXISTS viewed_posts (
    post_id TEXT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES actors(id) ON DELETE CASCADE,
    at BIGINT NOT NULL,
    PRIMARY KEY (post_id, user_id)
);