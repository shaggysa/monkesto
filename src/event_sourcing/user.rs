use leptos::prelude::ServerFnError;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashSet;
use uuid::Uuid;

use super::event::{AggregateType, EventType};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum UserEvent {
    Created {
        username: String,
        hashed_password: String,
    },
    UsernameUpdated {
        username: String,
    },
    PasswordUpdated {
        hashed_password: String,
    },
    LoggedIn {
        session_id: String,
    },
    LoggedOut {
        session_id: String,
    },
    CreatedJournal {
        id: Uuid,
    },
    InvitedToJournal {
        id: Uuid,
    },
    AcceptedJournalInvite {
        id: Uuid,
    },
    DeclinedJournalInvite {
        id: Uuid,
    },
    RemovedFromJournal {
        id: Uuid,
    },
    Deleted,
}

impl UserEvent {
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
        .bind(AggregateType::User)
        .bind(EventType::from_user_event(self))
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
pub struct UserState {
    pub id: Uuid,
    pub authenticated_sessions: std::collections::HashSet<String>,
    pub username: String,
    pub hashed_password: String,
    pub pending_journal_invites: HashSet<Uuid>,
    pub accepted_journal_invites: HashSet<Uuid>,
    pub owned_journals: HashSet<Uuid>,
    pub deleted: bool,
}

impl UserState {
    pub async fn from_events(id: Uuid, events: Vec<UserEvent>) -> Self {
        let mut aggregate = Self {
            id,
            ..Default::default()
        };

        for event in events {
            aggregate.apply(event).await;
        }
        aggregate
    }

    pub async fn apply(&mut self, event: UserEvent) {
        match event {
            UserEvent::Created {
                username,
                hashed_password: password,
            } => {
                self.username = username;
                self.hashed_password = password;
            }
            UserEvent::UsernameUpdated { username } => self.username = username,
            UserEvent::PasswordUpdated {
                hashed_password: password,
            } => self.hashed_password = password,
            UserEvent::LoggedIn { session_id } => {
                _ = self.authenticated_sessions.insert(session_id)
            }
            UserEvent::LoggedOut { session_id } => {
                _ = self.authenticated_sessions.remove(&session_id)
            }
            UserEvent::CreatedJournal { id } => _ = self.owned_journals.insert(id),
            UserEvent::InvitedToJournal { id } => _ = self.pending_journal_invites.insert(id),
            UserEvent::DeclinedJournalInvite { id } => _ = self.pending_journal_invites.remove(&id),
            UserEvent::AcceptedJournalInvite { id } => {
                if self.pending_journal_invites.remove(&id) {
                    _ = self.accepted_journal_invites.insert(id);
                }
            }
            UserEvent::RemovedFromJournal { id } => _ = self.accepted_journal_invites.remove(&id),
            UserEvent::Deleted => self.deleted = true,
        }
    }
}
