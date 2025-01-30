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

use crate::auth::get_user;

pub async fn route(
    map: HeaderMap,
    State(state): State<crate::GSt>,
    Path(other_user): Path<String>,
) -> Result<String, crate::Error> {
    let (actor, _) = get_user(&map, &state.key, &state.pg).await?;

    if !super::follow_exists(&state.pg, &actor.id, &other_user).await? {
        return Err(crate::Error::UserNotFollowed);
    }

    sqlx::query!(
        "DELETE FROM follows WHERE follower_id = $1 AND followee_id = $2;",
        &actor.id,
        &other_user
    )
    .execute(&state.pg)
    .await?;

    Ok("".to_string())
}
