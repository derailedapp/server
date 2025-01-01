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
    extract::{Query, State},
};
use db_models::{Post, Thread};
use serde::Deserialize;
use sqlx::types::chrono;

use crate::utils::get_thread;

#[derive(Deserialize)]
pub struct ScrollOptions {
    #[serde(default)]
    before_ts: Option<i64>,
}

pub async fn route(
    Query(options): Query<ScrollOptions>,
    State(state): State<crate::GSt>,
) -> Result<Json<Vec<Thread>>, crate::Error> {
    let ts = chrono::Utc::now().timestamp_millis();
    Ok(Json(
        futures::future::join_all(
            sqlx::query_as!(
                Post,
                "SELECT * FROM posts WHERE indexed_ts < $1 AND parent_id IS NULL ORDER BY indexed_ts DESC LIMIT 30;",
                &options.before_ts.unwrap_or(ts)
            )
            .fetch_all(&state.pg)
            .await?
            .into_iter()
            .map(|post| get_thread(&state.pg, post, false)),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<Thread>, crate::Error>>()?,
    ))
}
