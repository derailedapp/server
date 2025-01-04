/*
   Copyright 2024 V.J. De Chico

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
    Json,
    extract::{Path, State},
    http::HeaderMap,
};
use serde::Deserialize;

use crate::auth::get_user;

#[derive(Deserialize)]
pub struct ReactData {
    emoji: String,
}

pub async fn route(
    map: HeaderMap,
    State(state): State<crate::GSt>,
    Path(track_id): Path<String>,
    Json(model): Json<ReactData>,
) -> Result<String, crate::Error> {
    if emojis::get(&model.emoji).is_none() {
        return Err(crate::Error::EmojiNotSupported);
    }

    let (actor, _) = get_user(&map, &state.key, &state.pg).await?;

    let post = sqlx::query!("SELECT id FROM tracks WHERE id = $1", track_id)
        .fetch_optional(&state.pg)
        .await?;

    if let Some(post) = post {
        let existing_reaction = sqlx::query!(
            "SELECT emoji FROM track_reactions WHERE track_id = $1 AND user_id = $2 AND emoji = $3;",
            post.id,
            actor.id,
            &model.emoji
        )
            .fetch_optional(&state.pg)
            .await?;

        if existing_reaction.is_some() {
            sqlx::query!(
                "DELETE FROM track_reactions WHERE track_id = $1 AND user_id = $2 AND emoji = $3;",
                post.id,
                actor.id,
                &model.emoji
            )
            .execute(&state.pg)
            .await?;
        } else {
            return Err(crate::Error::ReactionNotExist);
        }

        Ok("".to_string())
    } else {
        Err(crate::Error::TrackNotExist)
    }
}
