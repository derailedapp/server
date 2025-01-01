CREATE TABLE IF NOT EXISTS viewed_tracks (
    track_id TEXT NOT NULL REFERENCES tracks(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES actors(id) ON DELETE CASCADE,
    at BIGINT NOT NULL,
    PRIMARY KEY (track_id, user_id)
);