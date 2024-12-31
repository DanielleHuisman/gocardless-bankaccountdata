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

/// JwtObtainPairRequest : Obtain JWT pair.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct JwtObtainPairRequest {
    /// Secret id from /user-secrets/
    #[serde(rename = "secret_id")]
    pub secret_id: String,
    /// Secret key from /user-secrets/
    #[serde(rename = "secret_key")]
    pub secret_key: String,
}

impl JwtObtainPairRequest {
    /// Obtain JWT pair.
    pub fn new(secret_id: String, secret_key: String) -> JwtObtainPairRequest {
        JwtObtainPairRequest {
            secret_id,
            secret_key,
        }
    }
}
