use anyhow::Result;

use crate::Client;

pub struct Employments {
    pub client: Client,
}

impl Employments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Employments { client }
    }

    /**
    * List employments.
    *
    * This function performs a `GET` to the `/v1/employments` endpoint.
    *
    * Lists all employments, except for the deleted ones.
    *
    * This endpoint requires and returns country-specific data. The exact required and returned fields will
    * vary depending on which country the employment is in. To see the list of parameters for each country,
    * see the **Show form schema** endpoint under the [Countries](#tag/Countries) category.
    *
    * Please note that the compliance requirements for each country are subject to change, which will result
    * in required and optional parameters being added and deleted without notice.
    *
    * If you are using this endpoint to build an integration, make sure you are dynamically collecting or
    * displaying the latest parameters for each country by querying the _"Show form schema"_ endpoint.
    *
    * For more information on JSON Schemas, see the **How JSON Schemas work** documentation.
    *
    * To learn how you can dynamically generate forms to display in your UI, see the documentation for
    * the [json-schema-form](https://www.notion.so/remotecom/json-schema-form-Documentation-4f390236948b4b2e8b7350ebcd488ca6) tool.
    *
    *
    *
    * **Parameters:**
    *
    * * `company_id: &str` -- Company ID.
    * * `page: i64` -- The current page among all of the total_pages.
    * * `page_size: i64` -- Change the amount of records returned per page, defaults to 20, limited to 100.
    */
    pub async fn get_index(
        &self,
        company_id: &str,
        page: i64,
        page_size: i64,
    ) -> Result<crate::types::ListEmploymentsResponse> {
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
        let url = format!("/v1/employments?{}", query_);

        self.client.get(&url, None).await
    }

    /**
    * Create employment.
    *
    * This function performs a `POST` to the `/v1/employments` endpoint.
    *
    * Creates an employment. We support creating employees and contractors.
    *
    * This endpoint requires and returns country-specific data. The exact required and returned fields will
    * vary depending on which country the employment is in. To see the list of parameters for each country,
    * see the **Show form schema** endpoint under the [Countries](#tag/Countries) category.
    *
    * Please note that the compliance requirements for each country are subject to change, which will result
    * in required and optional parameters being added and deleted without notice.
    *
    * If you are using this endpoint to build an integration, make sure you are dynamically collecting or
    * displaying the latest parameters for each country by querying the _"Show form schema"_ endpoint.
    *
    * For more information on JSON Schemas, see the **How JSON Schemas work** documentation.
    *
    * To learn how you can dynamically generate forms to display in your UI, see the documentation for
    * the [json-schema-form](https://www.notion.so/remotecom/json-schema-form-Documentation-4f390236948b4b2e8b7350ebcd488ca6) tool.
    *
    *
    */
    pub async fn post_create(
        &self,
        body: &crate::types::EmploymentBasicParams,
    ) -> Result<crate::types::EmploymentResponse> {
        let url = "/v1/employments".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Show employment.
    *
    * This function performs a `GET` to the `/v1/employments/{employment_id}` endpoint.
    *
    * Shows all the information of an employment.
    *
    * This endpoint requires and returns country-specific data. The exact required and returned fields will
    * vary depending on which country the employment is in. To see the list of parameters for each country,
    * see the **Show form schema** endpoint under the [Countries](#tag/Countries) category.
    *
    * Please note that the compliance requirements for each country are subject to change, which will result
    * in required and optional parameters being added and deleted without notice.
    *
    * If you are using this endpoint to build an integration, make sure you are dynamically collecting or
    * displaying the latest parameters for each country by querying the _"Show form schema"_ endpoint.
    *
    * For more information on JSON Schemas, see the **How JSON Schemas work** documentation.
    *
    * To learn how you can dynamically generate forms to display in your UI, see the documentation for
    * the [json-schema-form](https://www.notion.so/remotecom/json-schema-form-Documentation-4f390236948b4b2e8b7350ebcd488ca6) tool.
    *
    *
    *
    * **Parameters:**
    *
    * * `employment_id: &str` -- Employment ID.
    */
    pub async fn get_show(&self, employment_id: &str) -> Result<crate::types::EmploymentResponse> {
        let url = format!(
            "/v1/employments/{}",
            crate::progenitor_support::encode_path(&employment_id.to_string()),
        );

        self.client.get(&url, None).await
    }

    /**
    * Update employment.
    *
    * This function performs a `PUT` to the `/v1/employments/{employment_id}` endpoint.
    *
    * Updates an employment.
    *
    * **For `created` employments:** You can change all basic params and onboarding tasks or perform a per onboarding task update.
    *
    * **For `active` employments:** At this stage, it is only allowed to update the manager (`manager_id` field).
    *
    * This endpoint requires and returns country-specific data. The exact required and returned fields will
    * vary depending on which country the employment is in. To see the list of parameters for each country,
    * see the **Show form schema** endpoint under the [Countries](#tag/Countries) category.
    *
    * Please note that the compliance requirements for each country are subject to change, which will result
    * in required and optional parameters being added and deleted without notice.
    *
    * If you are using this endpoint to build an integration, make sure you are dynamically collecting or
    * displaying the latest parameters for each country by querying the _"Show form schema"_ endpoint.
    *
    * For more information on JSON Schemas, see the **How JSON Schemas work** documentation.
    *
    * To learn how you can dynamically generate forms to display in your UI, see the documentation for
    * the [json-schema-form](https://www.notion.so/remotecom/json-schema-form-Documentation-4f390236948b4b2e8b7350ebcd488ca6) tool.
    *
    *
    * Please contact Remote if you need to update contractors via API since it's currently not supported.
    *
    *
    * **Parameters:**
    *
    * * `employment_id: &str` -- Employment ID.
    */
    pub async fn patch_update_2(
        &self,
        employment_id: &str,
        body: &crate::types::EmploymentFullParams,
    ) -> Result<crate::types::EmploymentResponse> {
        let url = format!(
            "/v1/employments/{}",
            crate::progenitor_support::encode_path(&employment_id.to_string()),
        );

        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
    * Update employment.
    *
    * This function performs a `PATCH` to the `/v1/employments/{employment_id}` endpoint.
    *
    * Updates an employment.
    *
    * **For `created` employments:** You can change all basic params and onboarding tasks or perform a per onboarding task update.
    *
    * **For `active` employments:** At this stage, it is only allowed to update the manager (`manager_id` field).
    *
    * This endpoint requires and returns country-specific data. The exact required and returned fields will
    * vary depending on which country the employment is in. To see the list of parameters for each country,
    * see the **Show form schema** endpoint under the [Countries](#tag/Countries) category.
    *
    * Please note that the compliance requirements for each country are subject to change, which will result
    * in required and optional parameters being added and deleted without notice.
    *
    * If you are using this endpoint to build an integration, make sure you are dynamically collecting or
    * displaying the latest parameters for each country by querying the _"Show form schema"_ endpoint.
    *
    * For more information on JSON Schemas, see the **How JSON Schemas work** documentation.
    *
    * To learn how you can dynamically generate forms to display in your UI, see the documentation for
    * the [json-schema-form](https://www.notion.so/remotecom/json-schema-form-Documentation-4f390236948b4b2e8b7350ebcd488ca6) tool.
    *
    *
    * Please contact Remote if you need to update contractors via API since it's currently not supported.
    *
    *
    * **Parameters:**
    *
    * * `employment_id: &str` -- Employment ID.
    */
    pub async fn patch_update(
        &self,
        employment_id: &str,
        body: &crate::types::EmploymentFullParams,
    ) -> Result<crate::types::EmploymentResponse> {
        let url = format!(
            "/v1/employments/{}",
            crate::progenitor_support::encode_path(&employment_id.to_string()),
        );

        self.client
            .patch(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}
