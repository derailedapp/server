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

use argon2::{
    Argon2, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State, http::HeaderMap};
use models::UserProfile;
use serde::Deserialize;

use crate::{auth::get_user, utils::get_profile};

#[derive(Debug, Deserialize)]
pub struct EditSelf {
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    new_password: Option<String>,
    #[serde(default)]
    old_password: Option<String>,
    #[serde(default)]
    display_name: Option<Option<String>>,
    #[serde(default)]
    bio: Option<Option<String>>,
    #[serde(default)]
    status: Option<Option<String>>,
}

pub async fn route(
    map: HeaderMap,
    State(state): State<crate::GSt>,
    Json(model): Json<EditSelf>,
) -> Result<Json<UserProfile>, crate::Error> {
    let (mut actor, account) = get_user(&map, &state.key, &state.pg).await?;

    let mut tx = state.pg.begin().await?;

    if let Some(display_name) = model.display_name {
        sqlx::query!(
            "UPDATE actors SET display_name = $1 WHERE id = $2",
            display_name,
            &actor.id
        )
        .execute(&mut *tx)
        .await?;
        actor.display_name = display_name;
    }
    if let Some(bio) = model.bio {
        sqlx::query!("UPDATE actors SET bio = $1 WHERE id = $2", bio, &actor.id)
            .execute(&mut *tx)
            .await?;
        actor.bio = bio;
    }
    if let Some(status) = model.status {
        sqlx::query!(
            "UPDATE actors SET status = $1 WHERE id = $2",
            status,
            &actor.id
        )
        .execute(&mut *tx)
        .await?;
        actor.status = status;
    }

    let mut valid_password = false;

    // password-dependant
    let argon = Argon2::default();
    if let Some(old_password) = model.old_password {
        if argon
            .verify_password(
                old_password.as_bytes(),
                &argon2::PasswordHash::new(&account.password).unwrap(),
            )
            .is_err()
        {
            return Err(crate::Error::InvalidFormerPassword);
        } else {
            valid_password = true;
        }
    }
    if let Some(new_password) = model.new_password {
        if !valid_password {
            return Err(crate::Error::InvalidFormerPassword);
        }

        let salt = SaltString::generate(&mut OsRng);
        let password = argon
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|_| crate::Error::FailedPasswordHash)?
            .to_string();
        sqlx::query!(
            "UPDATE accounts SET password = $1 WHERE id = $2",
            password,
            &actor.id
        )
        .execute(&mut *tx)
        .await?;
    }
    if let Some(email) = model.email {
        if !valid_password {
            return Err(crate::Error::InvalidFormerPassword);
        }

        sqlx::query!(
            "UPDATE accounts SET email = $1 WHERE id = $2",
            email,
            &actor.id
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(Json(get_profile(&state.pg, actor).await?))
}
