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
    Json,
    body::Bytes,
    extract::{Multipart, State},
    http::HeaderMap,
};
use caesium::parameters::CSParameters;
use image::EncodableLayout;
use models::UserProfile;

use crate::{auth::get_user, utils::get_profile};

pub async fn route(
    map: HeaderMap,
    State(state): State<crate::GSt>,
    mut multipart: Multipart,
) -> Result<Json<UserProfile>, crate::Error> {
    let (user, _) = get_user(&map, &state.key, &state.pg).await?;
    let mut avatar_image = None;
    let mut banner_image = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("invalid").to_string();
        if !["avatar", "banner"].contains(&name.as_str()) {
            return Err(crate::Error::InvalidImageType);
        }

        let b = field.bytes().await?;
        if b == Bytes::from_owner("null".to_string()) {
            if name == "avatar" {
                avatar_image = Some(None)
            } else {
                banner_image = Some(None)
            };
        }

        let image = image::load_from_memory(b.as_bytes())?;

        let img = if name == "avatar" {
            image.resize_to_fill(256, 256, image::imageops::FilterType::Lanczos3)
        } else {
            image.resize_to_fill(1500, 500, image::imageops::FilterType::Lanczos3)
        };
        let mut writer = std::io::Cursor::new(Vec::new());
        img.write_to(&mut writer, image::ImageFormat::WebP)?;

        let webp = writer.into_inner();
        let mut parameters = CSParameters::new();
        parameters.keep_metadata = true;
        parameters.webp.quality = 55;

        let img = caesium::compress_in_memory(webp, &parameters).unwrap();

        if name == "avatar" {
            avatar_image = Some(Some(img));
        } else {
            banner_image = Some(Some(img));
        }
    }

    let mut tx = state.pg.begin().await?;

    if let Some(avatar) = avatar_image {
        if let Some(ref avatar_id) = user.avatar {
            state.avatars.delete_object(avatar_id).await?;
        }
        let new_id = state.snow.generate_lock_free().unwrap().to_string();

        if let Some(avatar) = avatar {
            state.avatars.put_object(&new_id, avatar.as_bytes()).await?;
        }
        let new_id = new_id + "//:dsm";
        sqlx::query!(
            "UPDATE actors SET avatar = $1 WHERE id = $2;",
            new_id,
            user.id
        )
        .execute(&mut *tx)
        .await?;
    }

    if let Some(banner) = banner_image {
        if let Some(ref banner_id) = user.banner {
            state.banners.delete_object(banner_id).await?;
        }
        let new_id = state.snow.generate_lock_free().unwrap().to_string();

        if let Some(banner) = banner {
            state.banners.put_object(&new_id, banner.as_bytes()).await?;
        }
        let new_id = new_id + "//:dsm";
        sqlx::query!(
            "UPDATE actors SET banner = $1 WHERE id = $2;",
            new_id,
            user.id
        )
        .execute(&mut *tx)
        .await?;
    }

    Ok(Json(get_profile(&state.pg, user).await?))
}
