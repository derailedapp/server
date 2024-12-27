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

use axum::{Json, extract::State, http::HeaderMap};
use db_models::Actor;
use serde::Serialize;

use crate::auth::get_user;

#[derive(Serialize)]
pub struct UserProfile {
    actor: Actor,
    followed: i64,
    followers: i64,
    posts: i64,
}

pub async fn route(
    map: HeaderMap,
    State(state): State<crate::GSt>,
) -> Result<Json<UserProfile>, crate::Error> {
    let (actor, _) = get_user(&map, &state.key, &state.pg).await?;

    // fetch metadata
    let followed_users = sqlx::query!(
        "SELECT COUNT(followee_id) FROM follows WHERE follower_id = $1;",
        &actor.id
    )
    .fetch_one(&state.pg)
    .await?;
    let followers = sqlx::query!(
        "SELECT COUNT(follower_id) FROM follows WHERE followee_id = $1;",
        &actor.id
    )
    .fetch_one(&state.pg)
    .await?;
    let posts = sqlx::query!(
        "SELECT COUNT(id) FROM posts WHERE author_id = $1;",
        &actor.id
    )
    .fetch_one(&state.pg)
    .await?;

    Ok(Json(UserProfile {
        actor,
        followed: followed_users.count.unwrap_or(0),
        followers: followers.count.unwrap_or(0),
        posts: posts.count.unwrap_or(0),
    }))
}
