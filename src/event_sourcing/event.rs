use super::account::AccountEvent;
use super::user::UserEvent;

#[derive(sqlx::Type)]
#[sqlx(type_name = "smallint")]
#[repr(i16)]
pub enum AggregateType {
    User = 1,
    Account = 2,
    Transaction = 3,
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
    UserAddedAccount = 6,
    UserDeleted = 7,

    // Account events (100-199)
    AccountCreated = 100,
    AccountAddedTenant = 101,
    AccountUpdatedTenant = 102,
    AccountRemovedTenant = 103,
    AccountBalanceUpdated = 104,
    AccountDeleted = 105,

    // Transaction events (200-299)
    TransactionPosted = 200,
}

impl EventType {
    pub fn from_user_event(user_event: &UserEvent) -> Self {
        match user_event {
            UserEvent::Created { .. } => Self::UserCreated,
            UserEvent::UsernameUpdated { .. } => Self::UsernameUpdated,
            UserEvent::PasswordUpdated { .. } => Self::UserPasswordUpdated,
            UserEvent::LoggedIn { .. } => Self::UserLoggedIn,
            UserEvent::LoggedOut { .. } => Self::UserLoggedOut,
            UserEvent::AddedAccount { .. } => Self::UserAddedAccount,
            UserEvent::Deleted => Self::UserDeleted,
        }
    }

    pub fn from_account_event(account_event: &AccountEvent) -> Self {
        match account_event {
            AccountEvent::Created { .. } => Self::AccountCreated,
            AccountEvent::AddedTenant { .. } => Self::AccountAddedTenant,
            AccountEvent::UpdateTenant { .. } => Self::AccountUpdatedTenant,
            AccountEvent::RemoveTenant { .. } => Self::AccountRemovedTenant,
            AccountEvent::BalanceUpdated { .. } => Self::AccountBalanceUpdated,
            AccountEvent::Deleted => Self::AccountDeleted,
        }
    }
}
