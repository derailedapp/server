/*
   Copyright 2024-2025 V.J. De Chico

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use axum::{
    extract::{Path, State},
    http::HeaderMap,
};
use sqlx::types::chrono;

use crate::{
    auth::get_user,
    utils::{get_channel, send_event},
};
use models::{Message, Room};

static MESSAGE_CONTENT: &str = "Hey, I just followed you back. That means we're friends now!";

pub async fn route(
    map: HeaderMap,
    State(state): State<crate::GSt>,
    Path(other_user): Path<String>,
) -> Result<String, crate::Error> {
    let (actor, _) = get_user(&map, &state.key, &state.pg).await?;

    if super::follow_exists(&state.pg, &actor.id, &other_user).await? || actor.id == other_user {
        return Err(crate::Error::UserFollowed);
    }

    let mut tx = state.pg.begin().await?;

    sqlx::query!(
        "INSERT INTO follows (follower_id, followee_id) VALUES ($1, $2);",
        &actor.id,
        &other_user
    )
    .execute(&mut *tx)
    .await?;

    if super::follow_exists(&state.pg, &other_user, &actor.id).await? {
        let (first_id, second_id) = if actor.id > other_user {
            (&actor.id, &other_user)
        } else {
            (&other_user, &actor.id)
        };
        let room_id =
            blake3::hash(("".to_string() + first_id.as_str() + second_id.as_str()).as_bytes())
                .to_string();

        // message info
        let ts = chrono::Utc::now().timestamp_millis();
        let message_content_hash = blake3::hash(MESSAGE_CONTENT.as_bytes()).to_string();
        let raw_msg_id = format!("{}/{}/{}/{}", &room_id, &actor.id, ts, message_content_hash);
        let message_id = blake3::hash(raw_msg_id.as_bytes()).to_string();

        let room = sqlx::query_as!(
            Room,
            "INSERT INTO rooms (id, type, last_message_id) VALUES ($1, 0, $2) RETURNING *",
            room_id,
            message_id
        )
        .fetch_one(&mut *tx)
        .await?;
        let msg = sqlx::query_as!(Message, "INSERT INTO messages (id, room_id, author_id, content, timestamp) VALUES ($1, $2, $3, $4, $5) RETURNING *;", message_id, room_id, actor.id, MESSAGE_CONTENT, ts)
            .fetch_one(&mut *tx)
            .await?;

        let channel = get_channel(&state.pg, room, None).await?;

        send_event(
            &state.consumants,
            vec![&actor.id, &other_user],
            crate::X15Message::RoomCreate {
                room: channel.room,
                members: channel.members,
            },
        )
        .await?;
        send_event(
            &state.consumants,
            vec![&actor.id, &other_user],
            crate::X15Message::MessageCreate { room_id, msg },
        )
        .await?;
    }

    Ok("".to_string())
}
