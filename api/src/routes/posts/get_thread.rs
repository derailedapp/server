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
use db_models::{Post, Thread};

use crate::utils::get_thread;

pub async fn route(
    State(state): State<crate::GSt>,
    Path(thread_id): Path<String>,
) -> Result<Json<Thread>, crate::Error> {
    let post = sqlx::query_as!(Post, "SELECT * FROM posts WHERE id = $1", thread_id)
        .fetch_optional(&state.pg)
        .await?;

    if let Some(post) = post {
        Ok(Json(get_thread(&state.pg, post, true).await?))
    } else {
        Err(crate::Error::PostNotExist)
    }
}
