mod general;
mod transact;

pub use general::GeneralJournal;
pub use transact::Transact;

use super::layout::Layout;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

#[derive(Debug, Clone)]
enum EntryType {
    Debit,
    Credit,
}

#[derive(Debug, Clone)]
struct Entry {
    pub account: AccountItem,
    pub amount: i64, // in cents
    pub entry_type: EntryType,
}

#[derive(Debug, Clone)]
struct AccountItem {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone)]
struct Person {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Clone)]
struct Transaction {
    pub id: Uuid,
    pub author: Person,
    pub entries: Vec<Entry>,
}

fn transactions() -> Vec<Transaction> {
    use std::str::FromStr;
    vec![
        Transaction {
            id: Uuid::from_str("350e8400-e29b-41d4-a716-446655440000").expect("Invalid UUID"),
            author: Person {
                id: Uuid::from_str("250e8400-e29b-41d4-a716-446655440000").expect("Invalid UUID"),
                username: "johndoe".to_string(),
            },
            entries: vec![
                Entry {
                    account: AccountItem {
                        id: Uuid::from_str("450e8400-e29b-41d4-a716-446655440000")
                            .expect("Invalid UUID"),
                        name: "Cash".to_string(),
                    },
                    amount: 4567, // $45.67 in cents
                    entry_type: EntryType::Credit,
                },
                Entry {
                    account: AccountItem {
                        id: Uuid::from_str("450e8400-e29b-41d4-a716-446655440002")
                            .expect("Invalid UUID"),
                        name: "Groceries Expense".to_string(),
                    },
                    amount: 4567, // $45.67 in cents
                    entry_type: EntryType::Debit,
                },
            ],
        },
        Transaction {
            id: Uuid::from_str("350e8400-e29b-41d4-a716-446655440001").expect("Invalid UUID"),
            author: Person {
                id: Uuid::from_str("250e8400-e29b-41d4-a716-446655440001").expect("Invalid UUID"),
                username: "janesmith".to_string(),
            },
            entries: vec![
                Entry {
                    account: AccountItem {
                        id: Uuid::from_str("450e8400-e29b-41d4-a716-446655440001")
                            .expect("Invalid UUID"),
                        name: "Checking Account".to_string(),
                    },
                    amount: 3214, // $32.14 in cents
                    entry_type: EntryType::Credit,
                },
                Entry {
                    account: AccountItem {
                        id: Uuid::from_str("450e8400-e29b-41d4-a716-446655440003")
                            .expect("Invalid UUID"),
                        name: "Fuel Expense".to_string(),
                    },
                    amount: 3214, // $32.14 in cents
                    entry_type: EntryType::Debit,
                },
            ],
        },
        Transaction {
            id: Uuid::from_str("350e8400-e29b-41d4-a716-446655440002").expect("Invalid UUID"),
            author: Person {
                id: Uuid::from_str("250e8400-e29b-41d4-a716-446655440002").expect("Invalid UUID"),
                username: "bobjohnson".to_string(),
            },
            entries: vec![
                Entry {
                    account: AccountItem {
                        id: Uuid::from_str("450e8400-e29b-41d4-a716-446655440000")
                            .expect("Invalid UUID"),
                        name: "Cash".to_string(),
                    },
                    amount: 425, // $4.25 in cents
                    entry_type: EntryType::Credit,
                },
                Entry {
                    account: AccountItem {
                        id: Uuid::from_str("450e8400-e29b-41d4-a716-446655440004")
                            .expect("Invalid UUID"),
                        name: "Coffee Expense".to_string(),
                    },
                    amount: 425, // $4.25 in cents
                    entry_type: EntryType::Debit,
                },
            ],
        },
    ]
}

fn journals() -> Vec<super::journal::Journal> {
    use std::str::FromStr;
    vec![
        super::journal::Journal {
            id: Uuid::from_str("550e8400-e29b-41d4-a716-446655440000").expect("Invalid UUID"),
            name: "Personal".to_string(),
        },
        super::journal::Journal {
            id: Uuid::from_str("550e8400-e29b-41d4-a716-446655440001").expect("Invalid UUID"),
            name: "Business".to_string(),
        },
    ]
}

#[component]
pub fn TransactionListPage() -> impl IntoView {
    let params = use_params_map();
    let journal_id = move || params.get().get("id").unwrap_or_default().to_string();

    let journal_name = move || {
        journals()
            .into_iter()
            .find(|j| j.id.to_string() == journal_id())
            .map(|j| j.name)
            .unwrap_or_else(|| "Unknown Journal".to_string())
    };

    view! {
        <Layout page_title=journal_name() show_switch_link=true journal_id=journal_id()>
            {transactions()
                .into_iter()
                .map(|transaction| {
                    view! {
                        <a
                            href=format!("/journal/{}/transaction/{}", journal_id(), transaction.id)
                            class="block p-4 bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors"
                        >
                            <div class="space-y-3">
                                <div class="space-y-2">
                                    {transaction
                                        .entries
                                        .iter()
                                        .map(|entry| {
                                            let entry_amount = format!(
                                                "${}.{:02}",
                                                entry.amount / 100,
                                                entry.amount % 100,
                                            );
                                            let entry_type_str = match entry.entry_type {
                                                EntryType::Debit => "Dr",
                                                EntryType::Credit => "Cr",
                                            };
                                            view! {
                                                <div class="flex justify-between items-center">
                                                    <span class="text-base font-medium text-gray-900 dark:text-white">
                                                        {entry.account.name.clone()}
                                                    </span>
                                                    <span class="text-base text-gray-700 dark:text-gray-300">
                                                        {entry_amount} " " {entry_type_str}
                                                    </span>
                                                </div>
                                            }
                                        })
                                        .collect_view()}
                                </div>
                                <div class="text-xs text-gray-400 dark:text-gray-500">
                                    {transaction.author.username}
                                </div>
                            </div>
                        </a>
                    }
                })
                .collect_view()}
            <hr class="mt-8 mb-6 border-gray-300 dark:border-gray-600" />
            <div class="mt-10">
                <div class="text-center p-8 bg-gray-50 dark:bg-gray-800 rounded-lg">
                    <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">
                        "Create New Transaction"
                    </h3>
                    <p class="text-sm text-gray-500 dark:text-gray-400">
                        "Transaction creation form will be implemented here"
                    </p>
                </div>
            </div>
        </Layout>
    }
}
