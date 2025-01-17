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

#![feature(duration_constructors)]

mod auth;
mod error;
mod routes;
mod snow;
mod utils;

use error::Error;
use s3::{Bucket, creds::Credentials};
use snow::SnowflakeGenerator;
use std::{env, sync::Arc, time::Duration};

use axum::http::Method;
use mimalloc::MiMalloc;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Clone)]
pub struct GSt {
    pub pg: PgPool,
    pub key: String,
    pub avatars: Bucket,
    pub banners: Bucket,
    pub snow: Arc<SnowflakeGenerator>,
}

pub const PICKLE_KEY: [u8; 32] = [0u8; 32];

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

static AVATARS_BUCKET_NAME: &str = "derailed-avatars";
static BANNERS_BUCKET_NAME: &str = "derailed-banners";

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:1234@localhost".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // s3 bucket
    let region = if let Ok(true) = std::env::var("R2_CF")
        .unwrap_or("false".to_string())
        .parse::<bool>()
    {
        s3::region::Region::R2 {
            account_id: std::env::var("CF_ACCOUNT_ID").unwrap(),
        }
    } else {
        s3::region::Region::Custom {
            region: std::env::var("S3_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            endpoint: std::env::var("S3_ENDPOINT").unwrap(),
        }
    };

    let credentials = Credentials::new(
        Some(&std::env::var("S3_ACCESS_KEY").unwrap()),
        Some(&std::env::var("S3_SECRET_KEY").unwrap()),
        None,
        None,
        None,
    )
    .expect("Failed to get S3 credentials");

    let avatars = Bucket::new(AVATARS_BUCKET_NAME, region.clone(), credentials.clone())
        .expect("Failed to get S3 avatars bucket");
    let banners = Bucket::new(BANNERS_BUCKET_NAME, region.clone(), credentials.clone())
        .expect("Failed to get S3 banners bucket");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PATCH])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = axum::Router::new()
        .merge(routes::router())
        .layer(cors)
        .with_state(GSt {
            pg: pool,
            key: env::var("JWT_SECRET_KEY")
                .expect("Could not find JWT secret key in environment variables"),
            avatars,
            banners,
            snow: Arc::new(snow::SnowflakeGenerator::default()),
        });

    let listener = TcpListener::bind("0.0.0.0:24650").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
