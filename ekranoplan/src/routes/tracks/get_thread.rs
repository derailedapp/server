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
    Json,
    extract::{Path, State},
    http::HeaderMap,
};
use models::{Thread, Track};

use crate::{auth::get_user, utils::get_thread};

pub async fn route(
    map: HeaderMap,
    State(state): State<crate::GSt>,
    Path(thread_id): Path<String>,
) -> Result<Json<Thread>, crate::Error> {
    let user = if map.contains_key("authorization") {
        let (user, _) = get_user(&map, &state.key, &state.pg).await?;
        Some(user)
    } else {
        None
    };

    let post = sqlx::query_as!(Track, "SELECT * FROM tracks WHERE id = $1", thread_id)
        .fetch_optional(&state.pg)
        .await?;

    if let Some(post) = post {
        Ok(Json(get_thread(&state.pg, post, true, &user).await?))
    } else {
        Err(crate::Error::TrackNotExist)
    }
}
