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

use std::{
    io::{self, IntoInnerError},
    string::FromUtf8Error,
};

use axum::{extract::multipart::MultipartError, http::header::ToStrError};

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
    MultipartError(#[from] MultipartError),

    #[error("Internal Server Error")]
    #[status(500)]
    ImageError(#[from] image::ImageError),

    #[error("Internal Server Error")]
    #[status(500)]
    S3Error(#[from] s3::error::S3Error),

    #[error("Internal Server Error")]
    #[status(500)]
    FailedPasswordHash,

    #[error("Internal Server Error")]
    #[status(500)]
    IdenticonError(#[from] identicon_rs::error::IdenticonError),

    #[error("Internal Server Error")]
    #[status(500)]
    UTF8Error(#[from] FromUtf8Error),

    #[error("Internal Server Error")]
    #[status(500)]
    BufWriterError(#[from] IntoInnerError<Vec<u8>>),

    #[error("Internal Server Error")]
    #[status(500)]
    CBORError(#[from] ciborium::ser::Error<io::Error>),

    #[error("Internal Server Error")]
    #[status(500)]
    SendError,

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

    #[error("Valid former password required")]
    #[status(400)]
    InvalidFormerPassword,

    #[error("User already followed")]
    #[status(400)]
    UserFollowed,

    #[error("User not followed")]
    #[status(400)]
    UserNotFollowed,

    #[error("User not found")]
    #[status(404)]
    UserNotFound,

    #[error("Track does not exist")]
    #[status(404)]
    TrackNotExist,

    #[error("Reaction does not exist")]
    #[status(400)]
    ReactionNotExist,

    #[error("Reaction already exists")]
    #[status(400)]
    ReactionExists,

    #[error("Image type not supported")]
    #[status(400)]
    InvalidImageType,

    #[error("Image not found")]
    #[status(400)]
    ImageNotFound,
}
