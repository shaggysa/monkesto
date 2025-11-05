use crate::event_sourcing::event::{DomainEvent, EventType};

use super::*;
use leptos::prelude::ServerFnError;
use sqlx::{query_scalar, types::JsonValue, PgPool};
use uuid::Uuid;

pub async fn fetch_user_id(username: String, pool: &PgPool) -> Result<Uuid, ServerFnError> {
    let uuid: Result<Option<Uuid>, sqlx::Error> = query_scalar(
        r#"
        SELECT aggregate_id FROM events
        WHERE (event_type = $1 OR event_type = $2) AND payload->'data'->>'username' = $3
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(event::EventType::UserCreated)
    .bind(event::EventType::UsernameUpdated)
    .bind(&username)
    .fetch_optional(pool)
    .await;

    match uuid {
        Ok(Some(id)) => Ok(id),
        Ok(None) => Err(ServerFnError::ServerError(format!(
            "Unable to find a user with the username {}",
            username
        ))),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

pub async fn fetch_username(user_id: Uuid, pool: &PgPool) -> Result<String, ServerFnError> {
    let username_events: Vec<JsonValue> = match query_scalar(
        r#"SELECT payload FROM events
            WHERE aggregate_id = $1 AND (event_type = $2 OR event_type = $3)
            ORDER BY created_at ASC
            "#,
    )
    .bind(user_id)
    .bind(EventType::UserCreated)
    .bind(EventType::UsernameUpdated)
    .fetch_all(pool)
    .await
    {
        Ok(s) => s,
        Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
    };

    let mut user = user::UserState {
        id: user_id,
        ..Default::default()
    };

    for raw_event in username_events {
        let event: DomainEvent = match serde_json::from_value(raw_event) {
            Ok(s) => s,
            Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
        };
        user.apply(event.to_user_event()?).await;
    }

    Ok(user.username)
}
