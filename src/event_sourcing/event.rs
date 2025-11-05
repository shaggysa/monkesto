use super::account::AccountEvent;
use super::journal::JournalEvent;
use super::user::UserEvent;
use leptos::prelude::ServerFnError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "payload")]
pub enum DomainEvent {
    User(UserEvent),
    Account(AccountEvent),
    Journal(JournalEvent),
}

impl DomainEvent {
    pub fn to_user_event(self) -> Result<UserEvent, ServerFnError> {
        match self {
            Self::User(s) => Ok(s),
            _ => Err(ServerFnError::ServerError(
                "failed to convert domain event to user event".to_string(),
            )),
        }
    }
    pub fn to_account_event(self) -> Result<AccountEvent, ServerFnError> {
        match self {
            Self::Account(s) => Ok(s),
            _ => Err(ServerFnError::ServerError(
                "failed to convert domain event to account event".to_string(),
            )),
        }
    }
    pub fn to_journal_event(self) -> Result<JournalEvent, ServerFnError> {
        match self {
            Self::Journal(s) => Ok(s),
            _ => Err(ServerFnError::ServerError(
                "failed to convert domain event to account event".to_string(),
            )),
        }
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "smallint")]
#[repr(i16)]
pub enum AggregateType {
    User = 1,
    Account = 2,
    Journal = 3,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "smallint")]
#[repr(i16)]
pub enum EventType {
    // User events (1-99)
    UserCreated = 1,
    UsernameUpdated = 2,
    UserPasswordUpdated = 3,
    UserLoggedIn = 4,
    UserLoggedOut = 5,
    UserCreatedJournal = 6,
    UserInvitedToJournal = 7,
    UserAcceptedJournalInvite = 8,
    UserDeclinedJournalInvite = 9,
    UserRemovedFromJournal = 10,
    UserDeleted = 11,

    // Account events (100-199)
    AccountCreated = 100,
    AccountBalanceUpdated = 101,
    AccountDeleted = 102,

    // Journal events (200-299)
    JournalCreated = 200,
    JournalAddedEntry = 201,
    JournalDeleted = 202,
}

impl EventType {
    pub fn from_user_event(user_event: &UserEvent) -> Self {
        match user_event {
            UserEvent::Created { .. } => Self::UserCreated,
            UserEvent::UsernameUpdated { .. } => Self::UsernameUpdated,
            UserEvent::PasswordUpdated { .. } => Self::UserPasswordUpdated,
            UserEvent::LoggedIn { .. } => Self::UserLoggedIn,
            UserEvent::LoggedOut { .. } => Self::UserLoggedOut,
            UserEvent::CreatedJournal { .. } => Self::UserCreatedJournal,
            UserEvent::InvitedToJournal { .. } => Self::UserInvitedToJournal,
            UserEvent::AcceptedJournalInvite { .. } => Self::UserAcceptedJournalInvite,
            UserEvent::DeclinedJournalInvite { .. } => Self::UserDeclinedJournalInvite,
            UserEvent::RemovedFromJournal { .. } => Self::UserRemovedFromJournal,
            UserEvent::Deleted => Self::UserDeleted,
        }
    }

    pub fn from_account_event(account_event: &AccountEvent) -> Self {
        match account_event {
            AccountEvent::Created { .. } => Self::AccountCreated,
            AccountEvent::BalanceUpdated { .. } => Self::AccountBalanceUpdated,
            AccountEvent::Deleted => Self::AccountDeleted,
        }
    }

    pub fn from_journal_event(journal_event: &JournalEvent) -> Self {
        match journal_event {
            JournalEvent::Created { .. } => Self::JournalCreated,
            JournalEvent::AddedEntry { .. } => Self::JournalAddedEntry,
            JournalEvent::Deleted => Self::JournalDeleted,
        }
    }
}
