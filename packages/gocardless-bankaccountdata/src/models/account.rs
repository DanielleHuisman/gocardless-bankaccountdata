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

/// Account : AccountSerializer.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    /// The ID of this Account, used to refer to this account in other API calls.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The date & time at which the account object was created.
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// The date & time at which the account object was last accessed.
    #[serde(rename = "last_accessed", skip_serializing_if = "Option::is_none")]
    pub last_accessed: Option<String>,
    /// The Account IBAN
    #[serde(rename = "iban", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    /// The Account BBAN
    #[serde(rename = "bban", skip_serializing_if = "Option::is_none")]
    pub bban: Option<String>,
    /// The processing status of this account.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The ASPSP associated with this account.
    #[serde(rename = "institution_id", skip_serializing_if = "Option::is_none")]
    pub institution_id: Option<String>,
    /// The name of the account owner.
    #[serde(rename = "owner_name", skip_serializing_if = "Option::is_none")]
    pub owner_name: Option<String>,
}

impl Account {
    /// AccountSerializer.
    pub fn new() -> Account {
        Account {
            id: None,
            created: None,
            last_accessed: None,
            iban: None,
            bban: None,
            status: None,
            institution_id: None,
            owner_name: None,
        }
    }
}
