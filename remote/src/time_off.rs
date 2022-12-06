use anyhow::Result;

use crate::Client;

pub struct TimeOff {
    pub client: Client,
}

impl TimeOff {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        TimeOff { client }
    }

    /**
    * List Time Off.
    *
    * This function performs a `GET` to the `/v1/timeoff` endpoint.
    *
    * Lists all Time Off records.
    *
    * **Parameters:**
    *
    * * `employment_id: &str` -- Only show time off for a specific employment.
    * * `timeoff_type: crate::types::TimeoffType` -- Filter time off by its type.
    * * `status: crate::types::GetIndexTimeoffStatus` -- Filter time off by its status.
    * * `order_by: crate::types::OrderBy` -- Sort order.
    * * `sort_by: crate::types::SortBy` -- Field to sort by.
    * * `page: i64` -- The current page among all of the total_pages.
    * * `page_size: i64` -- Change the amount of records returned per page, defaults to 20, limited to 100.
    */
    pub async fn get_index_timeoff(
        &self,
        employment_id: &str,
        timeoff_type: crate::types::TimeoffType,
        status: crate::types::GetIndexTimeoffStatus,
        order_by: crate::types::OrderBy,
        sort_by: crate::types::SortBy,
        page: i64,
        page_size: i64,
    ) -> Result<crate::types::ListTimeoffResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !employment_id.is_empty() {
            query_args.push(("employment_id".to_string(), employment_id.to_string()));
        }
        if !order_by.to_string().is_empty() {
            query_args.push(("order_by".to_string(), order_by.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        if !sort_by.to_string().is_empty() {
            query_args.push(("sort_by".to_string(), sort_by.to_string()));
        }
        if !status.to_string().is_empty() {
            query_args.push(("status".to_string(), status.to_string()));
        }
        if !timeoff_type.to_string().is_empty() {
            query_args.push(("timeoff_type".to_string(), timeoff_type.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/timeoff?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Create Time Off.
    *
    * This function performs a `POST` to the `/v1/timeoff` endpoint.
    *
    * Creates a Time Off record
    */
    pub async fn post_create_timeoff(
        &self,
        body: &crate::types::Company,
    ) -> Result<crate::types::TimeoffResponse> {
        let url = "/v1/timeoff".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * List Time Off Types.
    *
    * This function performs a `GET` to the `/v1/timeoff/types` endpoint.
    *
    * Lists all time off types that can be used for the `timeoff_type` parameter
    */
    pub async fn get_timeoff_types(&self) -> Result<crate::types::ListTimeoffTypesResponse> {
        let url = "/v1/timeoff/types".to_string();
        self.client.get(&url, None).await
    }

    /**
    * Show Time Off.
    *
    * This function performs a `GET` to the `/v1/timeoff/{id}` endpoint.
    *
    * Shows a single Time Off record
    *
    * **Parameters:**
    *
    * * `timeoff_id: &str` -- Timeoff ID.
    */
    pub async fn get_show_timeoff(
        &self,
        timeoff_id: &str,
    ) -> Result<crate::types::TimeoffResponse> {
        let url = format!(
            "/v1/timeoff/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Update Time Off.
    *
    * This function performs a `PUT` to the `/v1/timeoff/{id}` endpoint.
    *
    * Updates a Time Off record. This endpoint can also be used for cancelling a time off.
    */
    pub async fn patch_update_timeoff_2(
        &self,
        body: &crate::types::UpdateApprovedTimeoffParams,
    ) -> Result<crate::types::TimeoffResponse> {
        let url = format!(
            "/v1/timeoff/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Update Time Off.
    *
    * This function performs a `PATCH` to the `/v1/timeoff/{id}` endpoint.
    *
    * Updates a Time Off record. This endpoint can also be used for cancelling a time off.
    */
    pub async fn patch_update_timeoff(
        &self,
        body: &crate::types::UpdateApprovedTimeoffParams,
    ) -> Result<crate::types::TimeoffResponse> {
        let url = format!(
            "/v1/timeoff/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
