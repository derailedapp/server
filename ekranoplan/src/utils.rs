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

use std::{io::BufWriter, sync::Arc};

use axum::response::sse::Event;
use models::{Actor, Channel, ReadState, Room, RoomMember, Thread, Track, UserProfile};
use sqlx::PgPool;
use tokio::sync::RwLock;

use crate::{ConsumantsMap, X15Message};

pub async fn get_profile(pg: &PgPool, actor: Actor) -> Result<UserProfile, crate::Error> {
    // fetch metadata
    let followed_users = sqlx::query!(
        "SELECT COUNT(followee_id) FROM follows WHERE follower_id = $1;",
        &actor.id
    )
    .fetch_one(pg)
    .await?;
    let followers = sqlx::query!(
        "SELECT COUNT(follower_id) FROM follows WHERE followee_id = $1;",
        &actor.id
    )
    .fetch_one(pg)
    .await?;
    let tracks = sqlx::query!(
        "SELECT COUNT(id) FROM tracks WHERE author_id = $1 AND parent_id IS NULL;",
        &actor.id
    )
    .fetch_one(pg)
    .await?;

    Ok(UserProfile {
        actor,
        followed: followed_users.count.unwrap_or(0),
        followers: followers.count.unwrap_or(0),
        tracks: tracks.count.unwrap_or(0),
    })
}

pub async fn get_thread(
    pg: &PgPool,
    track: Track,
    get_children: bool,
    me: &Option<Actor>,
) -> Result<Thread, crate::Error> {
    let children = if get_children {
        // fetch a list of tracks
        let children = sqlx::query_as!(
            Track,
            "SELECT * FROM tracks WHERE parent_id = $1;",
            &track.id
        )
        .fetch_all(pg)
        .await?;

        // turn the tracks into threads
        let children = futures::future::join_all(
            children
                .into_iter()
                .map(|child| get_thread(pg, child, false, me)),
        )
        .await;

        // turn children from Vec<Result<Thread, Error>> to Result<Vec<Thread>, Error>
        let children: Result<Vec<Thread>, crate::Error> = children.into_iter().collect();
        Some(children?)
    } else {
        None
    };

    let profile = if let Some(ref author_id) = track.author_id {
        let user = sqlx::query_as!(Actor, "SELECT * FROM actors WHERE id = $1;", author_id)
            .fetch_one(pg)
            .await?;
        Some(get_profile(pg, user).await?)
    } else {
        None
    };
    let comments = sqlx::query!(
        "SELECT COUNT(id) FROM tracks WHERE parent_id = $1;",
        track.id
    )
    .fetch_one(pg)
    .await?;
    let likes = sqlx::query!(
        "SELECT COUNT(user_id) FROM track_reactions WHERE track_id = $1;",
        track.id
    )
    .fetch_one(pg)
    .await?;
    let bookmarks = sqlx::query!(
        "SELECT COUNT(user_id) FROM track_bookmarks WHERE track_id = $1;",
        track.id
    )
    .fetch_one(pg)
    .await?;

    let (bookmarked, liked) = if let Some(user) = me {
        let bookmarked = sqlx::query!(
            "SELECT * FROM track_bookmarks WHERE user_id = $1 AND track_id = $2;",
            user.id,
            &track.id
        )
        .fetch_optional(pg)
        .await?
        .is_some();

        let reaction = sqlx::query!(
            "SELECT user_id FROM track_reactions WHERE track_id = $1 AND user_id = $2;",
            &track.id,
            user.id
        )
        .fetch_optional(pg)
        .await?;
        (Some(bookmarked), Some(reaction.is_some()))
    } else {
        (None, None)
    };

    Ok(Thread {
        track,
        children,
        profile,
        bookmarked,
        liked,
        comments: comments.count.unwrap_or(0),
        likes: likes.count.unwrap_or(0),
        bookmarks: bookmarks.count.unwrap_or(0),
    })
}

#[inline(always)]
pub fn get_event(data: crate::X15Message) -> Result<Event, crate::Error> {
    let mut writer = BufWriter::new(Vec::new());
    ciborium::into_writer(&data, &mut writer)?;
    Ok(Event::default().data(String::from_utf8(writer.buffer().to_vec())?))
}

#[inline(always)]
pub async fn get_actor(pg: &PgPool, id: String) -> Result<Actor, crate::Error> {
    Ok(
        sqlx::query_as!(Actor, "SELECT * FROM actors WHERE id = $1;", id)
            .fetch_one(pg)
            .await?,
    )
}

pub async fn get_channel(
    pg: &PgPool,
    room: Room,
    actor: Option<&Actor>,
) -> Result<Channel, crate::Error> {
    let mems = sqlx::query_as!(
        RoomMember,
        "SELECT * FROM room_members WHERE room_id = $1;",
        room.id
    )
    .fetch_all(pg)
    .await?;

    let member_ids: Vec<String> = mems.into_iter().map(|m| m.actor_id).collect();

    // turn the actor ids into actors
    let members = futures::future::join_all(
        member_ids
            .into_iter()
            .map(|actor_id| get_actor(pg, actor_id)),
    )
    .await;

    // turn children from Vec<Result<Thread, Error>> to Result<Vec<Thread>, Error>
    let members: Result<Vec<Actor>, crate::Error> = members.into_iter().collect();
    let members = members?;

    let state = if let Some(actor) = actor {
        sqlx::query_as!(
            ReadState,
            "SELECT * FROM read_states WHERE room_id = $1 AND user_id = $2;",
            room.id,
            actor.id
        )
        .fetch_optional(pg)
        .await?
    } else {
        None
    };

    Ok(Channel {
        room,
        members,
        state,
    })
}

pub async fn send_event(
    consumants: &Arc<RwLock<ConsumantsMap>>,
    subjects: Vec<&str>,
    event: X15Message,
) -> Result<(), crate::Error> {
    let consumants = consumants.read().await;
    let real_event = get_event(event)?;
    for subject in subjects {
        if let Some(c) = consumants.get(subject) {
            for sender in c {
                sender
                    .send(Ok(real_event.clone()))
                    .await
                    .map_err(|_| crate::Error::SendError)?;
            }
        }
    }

    Ok(())
}
