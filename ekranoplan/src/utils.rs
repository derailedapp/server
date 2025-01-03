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

use models::{Actor, Reaction, Thread, Track, UserProfile};
use sqlx::PgPool;

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

pub async fn get_reactions(pg: &PgPool, post: &Track) -> Result<Vec<Reaction>, crate::Error> {
    let emojis = sqlx::query!(
        "SELECT DISTINCT emoji FROM track_reactions WHERE track_id = $1;",
        &post.id
    )
    .fetch_all(pg)
    .await?;

    let mut reactions = Vec::with_capacity(emojis.len());
    for emoji in emojis {
        let emoji_c = sqlx::query!(
            "SELECT COUNT(user_id) FROM track_reactions WHERE track_id = $1;",
            &emoji.emoji
        )
        .fetch_one(pg)
        .await?;
        reactions.push(Reaction {
            emoji: emoji.emoji,
            reactions: emoji_c.count.unwrap_or(0),
        });
    }

    Ok(reactions)
}

pub async fn get_thread(
    pg: &PgPool,
    track: Track,
    get_children: bool,
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
                .map(|child| get_thread(pg, child, false)),
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
    let reactions = get_reactions(pg, &track).await?;

    Ok(Thread {
        track,
        children,
        profile,
        reactions,
    })
}
