use super::event::{AggregateType, EventType};
use leptos::prelude::ServerFnError;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Hash)]
pub enum Permissions {
    Read,
    Write,
    Share,
    Delete,
}

#[derive(Serialize, Deserialize)]
pub struct BalanceUpdate {
    account_id: Uuid,
    changed_by: i64,
}

#[derive(Serialize, Deserialize)]
pub enum JournalEvent {
    Created { id: Uuid },
    AddedEntry { updates: Vec<BalanceUpdate> },
    Deleted,
}
impl JournalEvent {
    pub async fn push_db(&self, uuid: Uuid, pool: &PgPool) -> Result<i64, ServerFnError> {
        let payload = match serde_json::to_value(self) {
            Ok(s) => s,
            Err(e) => return Err(ServerFnError::ServerError(e.to_string())),
        };
        match sqlx::query_scalar(
            r#"
            INSERT INTO events (
                aggregate_id,
                aggregate_type,
                event_type,
                payload
            )
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
        )
        .bind(uuid)
        .bind(AggregateType::Journal)
        .bind(EventType::from_journal_event(self))
        .bind(payload)
        .fetch_one(pool)
        .await
        {
            Ok(s) => Ok(s),
            Err(e) => Err(ServerFnError::ServerError(e.to_string())),
        }
    }
}

#[derive(Default)]
pub struct JournalState {
    id: Uuid,
    transations: Vec<Vec<BalanceUpdate>>,
    deleted: bool,
}

impl JournalState {
    pub fn from_events(id: Uuid, events: Vec<JournalEvent>) -> Self {
        let mut aggregate = Self {
            id,
            ..Default::default()
        };

        for event in events {
            aggregate.apply(event);
        }
        aggregate
    }

    pub fn apply(&mut self, event: JournalEvent) {
        match event {
            JournalEvent::Created { id } => self.id = id,
            JournalEvent::AddedEntry { updates } => self.transations.push(updates),
            JournalEvent::Deleted => self.deleted = true,
        }
    }
}
