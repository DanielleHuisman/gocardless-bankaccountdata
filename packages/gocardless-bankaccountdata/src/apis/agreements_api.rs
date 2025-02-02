/*
 * GoCardless Bank Account Data API
 *
 * Securely access your user's bank account information for better lending, accounting, verification and financial management.
 *
 * The version of the OpenAPI document: 2.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};
use async_trait::async_trait;
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[async_trait]
pub trait AgreementsApi: Send + Sync {
    async fn accept_eua(
        &self,
        params: AcceptEuaParams,
    ) -> Result<models::EndUserAgreement, Error<AcceptEuaError>>;
    async fn create_eua(
        &self,
        params: CreateEuaParams,
    ) -> Result<models::EndUserAgreement, Error<CreateEuaError>>;
    async fn delete_eua_by_id(
        &self,
        params: DeleteEuaByIdParams,
    ) -> Result<(), Error<DeleteEuaByIdError>>;
    async fn retrieve_all_euas_for_an_end_user(
        &self,
        params: RetrieveAllEuasForAnEndUserParams,
    ) -> Result<models::PaginatedEndUserAgreementList, Error<RetrieveAllEuasForAnEndUserError>>;
    async fn retrieve_eua_by_id(
        &self,
        params: RetrieveEuaByIdParams,
    ) -> Result<models::EndUserAgreement, Error<RetrieveEuaByIdError>>;
}

pub struct AgreementsApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl AgreementsApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}

/// struct for passing parameters to the method [`accept_eua`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct AcceptEuaParams {
    /// A UUID string identifying this end user agreement.
    pub id: String,
    pub enduser_acceptance_details_request: models::EnduserAcceptanceDetailsRequest,
}

/// struct for passing parameters to the method [`create_eua`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct CreateEuaParams {
    pub end_user_agreement_request: models::EndUserAgreementRequest,
}

/// struct for passing parameters to the method [`delete_eua_by_id`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct DeleteEuaByIdParams {
    /// A UUID string identifying this end user agreement.
    pub id: String,
}

/// struct for passing parameters to the method [`retrieve_all_euas_for_an_end_user`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct RetrieveAllEuasForAnEndUserParams {
    /// Number of results to return per page.
    pub limit: Option<i32>,
    /// The initial zero-based index from which to return the results.
    pub offset: Option<i32>,
}

/// struct for passing parameters to the method [`retrieve_eua_by_id`]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "bon", derive(::bon::Builder))]
pub struct RetrieveEuaByIdParams {
    /// A UUID string identifying this end user agreement.
    pub id: String,
}

#[async_trait]
impl AgreementsApi for AgreementsApiClient {
    /// Accept an end-user agreement via the API
    async fn accept_eua(
        &self,
        params: AcceptEuaParams,
    ) -> Result<models::EndUserAgreement, Error<AcceptEuaError>> {
        let AcceptEuaParams {
            id,
            enduser_acceptance_details_request,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v2/agreements/enduser/{id}/accept/",
            local_var_configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        local_var_req_builder = local_var_req_builder.json(&enduser_acceptance_details_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<AcceptEuaError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// API endpoints related to end-user agreements.
    async fn create_eua(
        &self,
        params: CreateEuaParams,
    ) -> Result<models::EndUserAgreement, Error<CreateEuaError>> {
        let CreateEuaParams {
            end_user_agreement_request,
        } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v2/agreements/enduser/",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };
        local_var_req_builder = local_var_req_builder.json(&end_user_agreement_request);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<CreateEuaError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Delete an end user agreement
    async fn delete_eua_by_id(
        &self,
        params: DeleteEuaByIdParams,
    ) -> Result<(), Error<DeleteEuaByIdError>> {
        let DeleteEuaByIdParams { id } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v2/agreements/enduser/{id}/",
            local_var_configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeleteEuaByIdError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Overwrite pagination to map CONN consent data with end user agreements.  Args:     request (HttpRequest): Request  Returns:     HttpResponse: Response
    async fn retrieve_all_euas_for_an_end_user(
        &self,
        params: RetrieveAllEuasForAnEndUserParams,
    ) -> Result<models::PaginatedEndUserAgreementList, Error<RetrieveAllEuasForAnEndUserError>>
    {
        let RetrieveAllEuasForAnEndUserParams { limit, offset } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v2/agreements/enduser/",
            local_var_configuration.base_path
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_str) = limit {
            local_var_req_builder =
                local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_str) = offset {
            local_var_req_builder =
                local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
        }
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<RetrieveAllEuasForAnEndUserError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// Retrieve end user agreement by ID
    async fn retrieve_eua_by_id(
        &self,
        params: RetrieveEuaByIdParams,
    ) -> Result<models::EndUserAgreement, Error<RetrieveEuaByIdError>> {
        let RetrieveEuaByIdParams { id } = params;

        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!(
            "{}/api/v2/agreements/enduser/{id}/",
            local_var_configuration.base_path,
            id = crate::apis::urlencode(id)
        );
        let mut local_var_req_builder =
            local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder
                .header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
            local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<RetrieveEuaByIdError> =
                serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent {
                status: local_var_status,
                content: local_var_content,
                entity: local_var_entity,
            };
            Err(Error::ResponseError(local_var_error))
        }
    }
}

/// struct for typed errors of method [`accept_eua`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AcceptEuaError {
    Status405(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status401(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_eua`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEuaError {
    Status400(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status402(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_eua_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteEuaByIdError {
    Status400(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_all_euas_for_an_end_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveAllEuasForAnEndUserError {
    Status404(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_eua_by_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveEuaByIdError {
    Status404(models::ErrorResponse),
    Status400(models::ErrorResponse),
    Status429(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}
