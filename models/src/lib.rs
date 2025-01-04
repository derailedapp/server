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

use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Account {
    pub id: String,
    #[sqlx(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[sqlx(default)]
    #[serde(skip_serializing)]
    pub password: String,
    pub admin: bool,
    pub theme: String,
    #[serde(skip_serializing)]
    pub pickle: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserProfile {
    pub actor: Actor,
    pub followed: i64,
    pub followers: i64,
    pub tracks: i64,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Actor {
    pub id: String,
    // @vincentrps.example.com
    pub handle: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub status: Option<String>,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Reaction {
    pub emoji: String,
    pub reactions: i64,
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Track {
    pub id: String,
    pub r#type: i32,
    pub author_id: Option<String>,
    pub content: String,
    pub original_ts: i64,
    pub indexed_ts: i64,
    pub parent_id: Option<String>,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Bookmark {
    pub track_id: String,
    pub at: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Thread {
    pub track: Track,
    pub profile: Option<UserProfile>,
    pub reactions: Vec<Reaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Thread>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenResult {
    pub actor: Actor,
    pub account: Account,
    pub token: String,
}
