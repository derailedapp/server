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
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
};
use identicon_rs::Identicon;
use models::Actor;

use crate::auth::get_user;

pub async fn route(
    map: HeaderMap,
    State(state): State<crate::GSt>,
    Path(other_user): Path<String>,
) -> Result<impl IntoResponse, crate::Error> {
    let user = if other_user == "@me" {
        let (user, _) = get_user(&map, &state.key, &state.pg).await?;
        Some(user)
    } else {
        sqlx::query_as!(Actor, "SELECT * FROM actors WHERE id = $1;", other_user)
            .fetch_optional(&state.pg)
            .await?
    };

    if let Some(user) = user {
        if let Some(avatar) = user.avatar {
            let resp = state.avatars.get_object(avatar).await?;
            let mut headers = HeaderMap::new();
            headers.append("Content-Type", HeaderValue::from_static("image/webp"));
            Ok((headers, resp.to_vec()))
        } else {
            let mut ident = Identicon::new(&user.id);
            ident.set_mirrored(false);
            ident.set_border(15);
            let mut headers = HeaderMap::new();
            headers.append("Content-Type", HeaderValue::from_static("image/jpeg"));
            Ok((headers, ident.export_jpeg_data()?))
        }
    } else {
        Err(crate::Error::UserNotFound)
    }
}
