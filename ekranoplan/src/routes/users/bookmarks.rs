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
    http::HeaderMap,
};
use models::{Actor, Bookmark};

use crate::auth::get_user;

pub async fn route(
    map: HeaderMap,
    State(state): State<crate::GSt>,
    Path(other_user): Path<String>,
) -> Result<Json<Vec<Bookmark>>, crate::Error> {
    let user = if other_user == "@me" {
        let (user, _) = get_user(&map, &state.key, &state.pg).await?;
        Some(user)
    } else {
        sqlx::query_as!(Actor, "SELECT * FROM actors WHERE id = $1;", other_user)
            .fetch_optional(&state.pg)
            .await?
    };

    if let Some(user) = user {
        let bookmarks = sqlx::query!("SELECT * FROM track_bookmarks WHERE user_id = $1", &user.id)
            .fetch_all(&state.pg)
            .await?
            .into_iter()
            .map(|b| Bookmark {
                track_id: b.track_id,
                at: b.at,
            })
            .collect();
        Ok(Json(bookmarks))
    } else {
        Err(crate::Error::UserNotFound)
    }
}
