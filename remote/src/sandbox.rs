use anyhow::Result;

use crate::Client;

pub struct Sandbox {
    pub client: Client,
}

impl Sandbox {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Sandbox { client }
    }

    /**
    * Update employment in the Sandbox Environment.
    *
    * This function performs a `PUT` to the `/v1/sandbox/employments/{employment_id}` endpoint.
    *
    * Updates an employment. Use this endpoint to modify employment states for testing
    * in the Sandbox environment. This endpoint will respond with a 404 outside of the
    * Sandbox environment.
    *
    * For updating an employment's parameters outside of testing purposes, use [this
    * Employment update endpoint](#operation/patch_update_employment).
    *
    *
    * **Parameters:**
    *
    * * `employment_id: &str` -- Employment ID.
    */
    pub async fn patch_update_employment_4(
        &self,
        employment_id: &str,
        body: &crate::types::EmploymentUpdateParams,
    ) -> Result<crate::types::EmploymentResponse> {
        let url = format!(
            "/v1/sandbox/employments/{}",
            crate::progenitor_support::encode_path(&employment_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Update employment in the Sandbox Environment.
    *
    * This function performs a `PATCH` to the `/v1/sandbox/employments/{employment_id}` endpoint.
    *
    * Updates an employment. Use this endpoint to modify employment states for testing
    * in the Sandbox environment. This endpoint will respond with a 404 outside of the
    * Sandbox environment.
    *
    * For updating an employment's parameters outside of testing purposes, use [this
    * Employment update endpoint](#operation/patch_update_employment).
    *
    *
    * **Parameters:**
    *
    * * `employment_id: &str` -- Employment ID.
    */
    pub async fn patch_update_employment_3(
        &self,
        employment_id: &str,
        body: &crate::types::EmploymentUpdateParams,
    ) -> Result<crate::types::EmploymentResponse> {
        let url = format!(
            "/v1/sandbox/employments/{}",
            crate::progenitor_support::encode_path(&employment_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
