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
    extract::{Path, State},
    http::HeaderMap,
};
use sqlx::types::chrono;

use crate::auth::get_user;

pub async fn route(
    map: HeaderMap,
    State(state): State<crate::GSt>,
    Path(track_id): Path<String>,
) -> Result<String, crate::Error> {
    let (actor, _) = get_user(&map, &state.key, &state.pg).await?;

    let post = sqlx::query!("SELECT id FROM tracks WHERE id = $1", track_id)
        .fetch_optional(&state.pg)
        .await?;

    if let Some(post) = post {
        let time = chrono::Utc::now().timestamp_millis();
        sqlx::query!(
            "INSERT INTO viewed_tracks (track_id, user_id, at) VALUES ($1, $2, $3);",
            post.id,
            actor.id,
            time
        )
        .execute(&state.pg)
        .await?;
        Ok("".to_string())
    } else {
        Err(crate::Error::TrackNotExist)
    }
}