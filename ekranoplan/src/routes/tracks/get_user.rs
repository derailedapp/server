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
};
use models::{Thread, Track};

use crate::utils::get_thread;

pub async fn route(
    State(state): State<crate::GSt>,
    Path(other_user): Path<String>,
) -> Result<Json<Vec<Thread>>, crate::Error> {
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
            .map(|post| get_thread(&state.pg, post, false)),
        )
        .await
        .into_iter()
        .collect::<Result<Vec<Thread>, crate::Error>>()?,
    ))
}
