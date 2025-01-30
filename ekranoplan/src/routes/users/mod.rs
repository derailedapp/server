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
    extract::DefaultBodyLimit,
    routing::{get, patch, post},
};
use sqlx::PgPool;

pub mod avatar;
pub mod banner;
pub mod bookmarks;
pub mod edit;
pub mod follow;
pub mod get_self;
pub mod login;
pub mod new_assets;
pub mod profile;
pub mod register;
pub mod unfollow;

pub async fn follow_exists(
    db: &PgPool,
    follower: &str,
    followee: &str,
) -> Result<bool, crate::Error> {
    if sqlx::query!(
        "SELECT since FROM follows WHERE follower_id = $1 AND followee_id = $2",
        follower,
        followee
    )
    .fetch_optional(db)
    .await?
    .is_some()
    {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn router() -> axum::Router<crate::GSt> {
    axum::Router::new()
        .route("/create", post(register::route))
        .route("/login", post(login::route))
        .route(
            "/users/:user_id/follow",
            post(follow::route).delete(unfollow::route),
        )
        .route("/users/:user_id", get(profile::route))
        .route("/users/:user_id/avatar", get(avatar::route))
        .route("/users/:user_id/banner", get(banner::route))
        .route("/users/:user_id/bookmarks", get(bookmarks::route))
        .route("/users/@me", patch(edit::route).get(get_self::route))
        .route("/users/@me/assets", patch(new_assets::route))
        .layer(DefaultBodyLimit::max(14_680_064))
}
