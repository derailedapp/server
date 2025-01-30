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

use std::time::Duration;

use axum::{
    extract::State,
    http::HeaderMap,
    response::{Sse, sse::Event},
    routing::get,
};
use futures::Stream;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use crate::{auth::get_user, utils::get_event};

pub fn router() -> axum::Router<crate::GSt> {
    axum::Router::new().route("/x15", get(x15_handler))
}

pub async fn x15_handler(
    map: HeaderMap,
    State(state): State<crate::GSt>,
) -> Result<Sse<impl Stream<Item = Result<Event, crate::Error>>>, crate::Error> {
    let (actor, account) = get_user(&map, &state.key, &state.pg).await?;

    let stream = {
        let (sender, stream) = mpsc::channel(3_000);
        let mut consumants = state.consumants.write().await;
        if let Some(cons) = consumants.get_mut(&actor.id) {
            cons.push(sender.clone());
        }
        sender
            .send(get_event(crate::X15Message::Ready { actor, account }))
            .await
            .map_err(|_| crate::Error::SendError)?;
        stream
    };

    Ok(
        Sse::new(ReceiverStream::<Result<Event, crate::Error>>::new(stream)).keep_alive(
            axum::response::sse::KeepAlive::new()
                .interval(Duration::from_secs(30))
                .text(state.snow.generate().unwrap().to_string()),
        ),
    )
}
