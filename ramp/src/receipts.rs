use anyhow::Result;

use crate::Client;

pub struct Receipts {
    client: Client,
}

impl Receipts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Receipts { client }
    }

    /**
     * List receipts.
     *
     * This function performs a `GET` to the `/receipts` endpoint.
     *
     * Returns description of all receipts of a business.
     *
     * **Parameters:**
     *
     * * `from_date: chrono::DateTime<chrono::Utc>` -- Filter for receipts related to transactions which occurred after the specified date.
     * * `to_date: chrono::DateTime<chrono::Utc>` -- Filter for receipts related to transactions which occurred before the specified date.
     * * `created_after: chrono::DateTime<chrono::Utc>` -- Filter for receipts that were created after the specified date.
     * * `created_before: chrono::DateTime<chrono::Utc>` -- Filter for receipts that were created before the specified date.
     * * `start: &str` -- The ID of the last entity of the previous page, used for pagination to get the next page.
     * * `page_size: f64` -- The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default will be 1,000.
     */
    pub async fn get_page(
        &self,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
        created_after: Option<chrono::DateTime<chrono::Utc>>,
        created_before: Option<chrono::DateTime<chrono::Utc>>,
        start: &str,
        page_size: f64,
    ) -> Result<Vec<crate::types::Receipt>> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if let Some(date) = created_after {
            query_args.push(format!("created_after={}", &date.to_rfc3339()));
        }
        if let Some(date) = created_before {
            query_args.push(format!("created_before={}", &date.to_rfc3339()));
        }
        if let Some(date) = from_date {
            query_args.push(format!("from_date={}", &date.to_rfc3339()));
        }
        query_args.push(format!("page_size={}", page_size));
        if !start.is_empty() {
            query_args.push(format!("start={}", start));
        }
        if let Some(date) = to_date {
            query_args.push(format!("to_date={}", &date.to_rfc3339()));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/receipts?{}", query_);

        let resp: crate::types::GetReceiptsResponse = self.client.get(&url, None).await.unwrap();

        // Return our response data.
        Ok(resp.data)
    }

    /**
     * List receipts.
     *
     * This function performs a `GET` to the `/receipts` endpoint.
     *
     * As opposed to `get`, this function returns all the pages of the request at once.
     *
     * Returns description of all receipts of a business.
     */
    pub async fn get_all(
        &self,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
        created_after: Option<chrono::DateTime<chrono::Utc>>,
        created_before: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<Vec<crate::types::Receipt>> {
        let mut query_ = String::new();
        let mut query_args: Vec<String> = Default::default();
        if let Some(date) = created_after {
            query_args.push(format!("created_after={}", &date.to_rfc3339()));
        }
        if let Some(date) = created_before {
            query_args.push(format!("created_before={}", &date.to_rfc3339()));
        }
        if let Some(date) = from_date {
            query_args.push(format!("from_date={}", &date.to_rfc3339()));
        }
        if let Some(date) = to_date {
            query_args.push(format!("to_date={}", &date.to_rfc3339()));
        }
        for (i, n) in query_args.iter().enumerate() {
            if i > 0 {
                query_.push('&');
            }
            query_.push_str(n);
        }
        let url = format!("/receipts?{}", query_);

        let mut resp: crate::types::GetReceiptsResponse =
            self.client.get(&url, None).await.unwrap();

        let mut data = resp.data;
        let mut page = resp.page.next;

        // Paginate if we should.
        while !page.is_empty() {
            resp = self
                .client
                .get(page.trim_start_matches(crate::DEFAULT_HOST), None)
                .await
                .unwrap();

            data.append(&mut resp.data);

            if !resp.page.next.is_empty() && resp.page.next != page {
                page = resp.page.next.to_string();
            } else {
                page = "".to_string();
            }
        }

        // Return our response data.
        Ok(data)
    }

    /**
     * Get details for one receipt.
     *
     * This function performs a `GET` to the `/receipts/{id}` endpoint.
     *
     *
     */
    pub async fn get_receipt(&self, id: &str) -> Result<crate::types::Receipt> {
        let url = format!(
            "/receipts/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.get(&url, None).await
    }
}
