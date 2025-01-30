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
    Path(mut other_user): Path<String>,
) -> Result<Json<Vec<Thread>>, crate::Error> {
    let user = if map.contains_key("authorization") && other_user == "@me" {
        let (user, _) = get_user(&map, &state.key, &state.pg).await?;
        other_user = user.id.clone();
        Some(user)
    } else {
        None
    };

    Ok(Json(
        futures::future::join_all(
            sqlx::query_as!(
                Track,
                "SELECT * FROM tracks WHERE author_id = $1 AND parent_id IS NULL ORDER BY indexed_ts DESC;",
                other_user
            )
            .fetch_all(&state.pg)
            .await?
            .into_iter()
            .map(|post| get_thread(&state.pg, post, false, &user)),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<Thread>, crate::Error>>()?,
    ))
}
