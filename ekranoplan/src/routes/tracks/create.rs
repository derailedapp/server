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
use models::Track;
use serde::Deserialize;
use sqlx::types::chrono;

use crate::auth::get_user;

#[derive(Deserialize)]
pub struct CreatePost {
    content: String,
    #[serde(default)]
    parent_id: Option<String>,
}

pub async fn route(
    map: HeaderMap,
    State(state): State<crate::GSt>,
    Json(model): Json<CreatePost>,
) -> Result<Json<Track>, crate::Error> {
    let (actor, account) = get_user(&map, &state.key, &state.pg).await?;

    let pickle =
        vodozemac::olm::AccountPickle::from_encrypted(&account.pickle, &crate::PICKLE_KEY)?;
    let acc = vodozemac::olm::Account::from_pickle(pickle);

    let id = nanoid::nanoid!();
    let ts = chrono::Utc::now().timestamp_millis();

    let sig_fmt = format!("{}{}{}{}", &id, &actor.id, &ts, &model.content);

    let sig = acc.sign(sig_fmt).to_base64();

    // TODO: verify post id and return a prompt error
    Ok(Json(sqlx::query_as!(
        Track,
        "INSERT INTO tracks (id, type, author_id, content, original_ts, indexed_ts, parent_id, signature) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *;",
        id,
        0,
        actor.id,
        &model.content,
        &ts,
        &ts,
        model.parent_id,
        sig
    ).fetch_one(&state.pg).await?))
}
