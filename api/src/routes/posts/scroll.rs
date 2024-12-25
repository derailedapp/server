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
use bevy_db::Post;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ScrollOptions {
    exclude: Vec<String>,
}

pub async fn route(
    Query(options): Query<ScrollOptions>,
    State(state): State<crate::GSt>,
) -> Result<Json<Vec<Post>>, crate::Error> {
    Ok(Json(
        sqlx::query_as!(
            Post,
            "SELECT * FROM posts WHERE id != ANY($1) ORDER BY indexed_ts;",
            &options.exclude
        )
        .fetch_all(&state.pg)
        .await?,
    ))
}
