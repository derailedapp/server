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

use std::time::Duration;

use argon2::{Argon2, PasswordVerifier};
use axum::{Json, extract::State};
use db_models::{Account, Actor, TokenResult};
use jsonwebtoken::EncodingKey;
use serde::Deserialize;
use sqlx::types::chrono;

use crate::auth::Claims;

#[derive(Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

pub async fn route(
    State(state): State<crate::GSt>,
    Json(model): Json<Login>,
) -> Result<Json<db_models::TokenResult>, crate::Error> {
    let account = sqlx::query_as!(
        Account,
        "SELECT * FROM accounts WHERE email = $1;",
        model.email
    )
    .fetch_optional(&state.pg)
    .await?;

    if let Some(account) = account {
        let argon = Argon2::default();
        if argon
            .verify_password(
                model.password.as_bytes(),
                &argon2::PasswordHash::new(&account.password).unwrap(),
            )
            .is_ok()
        {
            let actor = sqlx::query_as!(Actor, "SELECT * FROM actors WHERE id = $1;", &account.id)
                .fetch_one(&state.pg)
                .await?;
            let session_id = nanoid::nanoid!();

            sqlx::query!(
                "INSERT INTO sessions (id, user_id) VALUES ($1, $2);",
                &session_id,
                &actor.id
            )
            .execute(&state.pg)
            .await?;

            let time = chrono::Utc::now().timestamp_millis() as u128;
            Ok(Json(TokenResult {
                actor,
                account,
                token: Claims {
                    sub: session_id,
                    exp: (time + Duration::from_weeks(6).as_millis())
                        .try_into()
                        .unwrap(),
                    iat: time as usize,
                }
                .make_token(&EncodingKey::from_secret(state.key.as_bytes()))?,
            }))
        } else {
            Err(crate::Error::Argon2Error)
        }
    } else {
        Err(crate::Error::Argon2Error)
    }
}
