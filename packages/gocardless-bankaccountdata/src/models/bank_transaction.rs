/*
 * GoCardless Bank Account Data API
 *
 * Securely access your user's bank account information for better lending, accounting, verification and financial management.
 *
 * The version of the OpenAPI document: 2.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// BankTransaction : BankTransactionSerializer.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BankTransaction {
    #[serde(rename = "booked")]
    pub booked: Vec<models::TransactionSchema>,
    #[serde(rename = "pending", skip_serializing_if = "Option::is_none")]
    pub pending: Option<Vec<models::TransactionSchema>>,
}

impl BankTransaction {
    /// BankTransactionSerializer.
    pub fn new(booked: Vec<models::TransactionSchema>) -> BankTransaction {
        BankTransaction {
            booked,
            pending: None,
        }
    }
}
