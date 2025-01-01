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
use models::UserProfile;

use crate::{auth::get_user, utils::get_profile};

pub async fn route(
    map: HeaderMap,
    State(state): State<crate::GSt>,
) -> Result<Json<UserProfile>, crate::Error> {
    let (actor, _) = get_user(&map, &state.key, &state.pg).await?;

    Ok(Json(get_profile(&state.pg, actor).await?))
}
