CREATE TABLE IF NOT EXISTS rooms (
    -- #12345678abc.homeserver.org
    id TEXT NOT NULL PRIMARY KEY,
    name TEXT,
    type INTEGER NOT NULL,
    last_message_id TEXT
);

CREATE TABLE IF NOT EXISTS room_members (
    room_id TEXT NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    actor_id TEXT NOT NULL REFERENCES actors(id),
    PRIMARY KEY (room_id, actor_id)
);

CREATE TABLE IF NOT EXISTS messages (
    -- !123abc#12345678abc.homeserver.org
    id TEXT NOT NULL PRIMARY KEY,
    room_id TEXT NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    author_id TEXT REFERENCES actors(id) ON DELETE SET NULL,
    content TEXT NOT NULL,
    timestamp BIGINT NOT NULL,
    edited_timestamp BIGINT
);

CREATE TABLE IF NOT EXISTS read_states (
    user_id TEXT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    room_id TEXT NOT NULL REFERENCES rooms(id) ON DELETE CASCADE,
    last_message_id TEXT,
    mentions INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, room_id)
);
