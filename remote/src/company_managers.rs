use anyhow::Result;

use crate::Client;

pub struct CompanyManagers {
    pub client: Client,
}

impl CompanyManagers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        CompanyManagers { client }
    }

    /**
    * List Company Managers.
    *
    * This function performs a `GET` to the `/v1/company-managers` endpoint.
    *
    * List all company managers of an integration. If filtered by the company_id param,
    * it lists only company managers belonging to the specified company.
    *
    *
    * **Parameters:**
    *
    * * `company_id: &str` -- A Company ID to filter the results (only applicable for Integration Partners).
    * * `page: i64` -- The current page among all of the total_pages.
    * * `page_size: i64` -- Change the amount of records returned per page, defaults to 20, limited to 100.
    */
    pub async fn get_index(
        &self,
        company_id: &str,
        page: i64,
        page_size: i64,
    ) -> Result<crate::types::CompanyManagersResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !company_id.is_empty() {
            query_args.push(("company_id".to_string(), company_id.to_string()));
        }
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!("/v1/company-managers?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Create and invite a Company Manager.
    *
    * This function performs a `POST` to the `/v1/company-managers` endpoint.
    *
    * Create a Company Manager and sends the invitation email for signing in to the Remote Platform.
    */
    pub async fn post_create(
        &self,
        body: &crate::types::CompanyManagerParams,
    ) -> Result<crate::types::CompanyManagerCreatedResponse> {
        let url = "/v1/company-managers".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
