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

use axum::http::header::ToStrError;

#[derive(Debug, thiserror::Error, axum_thiserror::ErrorStatus)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[error("Internal Server Error")]
    #[status(500)]
    DBError(#[from] sqlx::Error),

    #[error("Internal Server Error")]
    #[status(500)]
    VodoError(#[from] vodozemac::KeyError),

    #[error("Internal Server Error")]
    #[status(500)]
    VodoError2(#[from] vodozemac::PickleError),

    #[error("Internal Server Error")]
    #[status(500)]
    ToStrError(#[from] ToStrError),

    #[error("Internal Server Error")]
    #[status(500)]
    FailedPasswordHash,

    #[error("Invalid Token")]
    #[status(401)]
    InvalidToken(#[from] jsonwebtoken::errors::Error),

    #[error("Invalid Token")]
    #[status(401)]
    BadToken,

    #[error("Expired Session")]
    #[status(401)]
    ExpiredSession,

    #[error("Invalid email or password")]
    #[status(401)]
    Argon2Error,

    #[error("User already followed")]
    #[status(400)]
    UserFollowed,

    #[error("User not followed")]
    #[status(400)]
    UserNotFollowed,

    #[error("User not found")]
    #[status(404)]
    UserNotFound,

    #[error("Post does not exist")]
    #[status(404)]
    PostNotExist,
}
