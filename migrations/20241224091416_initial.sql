CREATE TABLE IF NOT EXISTS actors (
    id TEXT NOT NULL PRIMARY KEY,
    handle TEXT UNIQUE,
    display_name TEXT,
    bio TEXT,
    status TEXT,
    public_key TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS follows (
    follower_id TEXT NOT NULL REFERENCES actors(id),
    followee_id TEXT NOT NULL REFERENCES actors(id),
    since BIGINT NOT NULL,
    PRIMARY KEY (follower_id, followee_id)
);
CREATE TABLE IF NOT EXISTS accounts (
    id TEXT NOT NULL PRIMARY KEY REFERENCES actors(id) ON DELETE CASCADE,
    email TEXT UNIQUE,
    password TEXT NOT NULL,
    admin BOOLEAN NOT NULL DEFAULT false,
    theme TEXT NOT NULL,
    pickle TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT NOT NULL,
    user_id TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    PRIMARY KEY (id, user_id)
);
CREATE TABLE IF NOT EXISTS tracks (
    -- abcde1234/track_id
    id TEXT NOT NULL PRIMARY KEY,
    -- 0: Thread
    -- 1: Repost
    type INTEGER NOT NULL,
    author_id TEXT REFERENCES actors(id) ON DELETE SET NULL,
    content TEXT NOT NULL,
    original_ts BIGINT NOT NULL,
    indexed_ts BIGINT NOT NULL,
    -- abcde1234/track_id
    parent_id TEXT REFERENCES tracks(id) ON DELETE CASCADE,
    signature TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS track_reactions (
    track_id TEXT NOT NULL REFERENCES tracks(id),
    user_id TEXT NOT NULL REFERENCES actors(id),
    emoji TEXT NOT NULL,
    PRIMARY KEY (track_id, user_id)
);