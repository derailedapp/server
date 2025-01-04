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

use axum::routing::{get, post};

pub mod bookmark;
pub mod create;
pub mod delete;
pub mod get_thread;
pub mod get_user;
pub mod mark_viewed;
pub mod scroll;
pub mod unbookmark;

pub fn router() -> axum::Router<crate::GSt> {
    axum::Router::new()
        .route("/users/:user_id/tracks", get(get_user::route))
        .route("/tracks", post(create::route))
        .route(
            "/tracks/:track_id",
            get(get_thread::route).delete(delete::route),
        )
        .route("/tracks/:track_id/mark", post(mark_viewed::route))
        .route(
            "/tracks/:track_id/bookmark",
            post(bookmark::route).delete(unbookmark::route),
        )
        .route("/tracks/scroll", get(scroll::route))
}
