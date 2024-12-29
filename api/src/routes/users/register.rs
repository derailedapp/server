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

use argon2::{
    Argon2, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State};
use db_models::{Account, Actor, TokenResult};
use jsonwebtoken::EncodingKey;
use serde::Deserialize;
use serde_valid::Validate;
use sqlx::types::chrono;

use crate::auth::Claims;

#[derive(Deserialize, Validate)]
pub struct Register {
    pub email: String,
    pub password: String,
}

pub async fn route(
    State(state): State<crate::GSt>,
    Json(model): Json<Register>,
) -> Result<Json<db_models::TokenResult>, crate::Error> {
    let mut tx = state.pg.begin().await?;

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(model.password.as_bytes(), &salt)
        .map_err(|_| crate::Error::FailedPasswordHash)?
        .to_string();

    let user_id = nanoid::nanoid!();

    let acc = vodozemac::olm::Account::new();
    let public_key = acc.ed25519_key().to_base64();
    let pickle = acc.pickle().encrypt(&crate::PICKLE_KEY);

    let actor = sqlx::query_as!(
        Actor,
        "INSERT INTO actors (id, public_key) VALUES ($1, $2) RETURNING *;",
        &user_id,
        public_key
    )
    .fetch_one(&mut *tx)
    .await?;
    let account = sqlx::query_as!(Account, "INSERT INTO accounts (id, email, password, admin, theme, pickle) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;", &user_id, model.email, password_hash, false, "dark", pickle).fetch_one(&mut *tx).await?;

    let session_id = nanoid::nanoid!();

    sqlx::query!(
        "INSERT INTO sessions (id, user_id) VALUES ($1, $2);",
        &session_id,
        &actor.id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

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
}
