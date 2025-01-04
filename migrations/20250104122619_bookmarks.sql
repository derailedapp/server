CREATE TABLE IF NOT EXISTS track_bookmarks (
    user_id TEXT NOT NULL REFERENCES actors(id),
    track_id TEXT NOT NULL REFERENCES tracks(id),
    at BIGINT NOT NULL
);