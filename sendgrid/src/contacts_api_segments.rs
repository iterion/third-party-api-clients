use anyhow::Result;

use crate::Client;

pub struct ContactsApiSegments {
    pub client: Client,
}

impl ContactsApiSegments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        ContactsApiSegments { client }
    }

    /**
     * Retrieve all segments.
     *
     * This function performs a `GET` to the `/contactdb/segments` endpoint.
     *
     * **This endpoint allows you to retrieve all of your segments.**
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_segments(
        &self,
        on_behalf_of: &str,
    ) -> Result<crate::types::ListAllSegmentsResponse> {
        let url = "/contactdb/segments".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Create a Segment.
     *
     * This function performs a `POST` to the `/contactdb/segments` endpoint.
     *
     * **This endpoint allows you to create a new segment.**
     *
     *
     *   Valid operators for create and update depend on the type of the field for which you are searching.
     *
     * **Dates**
     * - "eq", "ne", "lt" (before), "gt" (after)
     *     - You may use MM/DD/YYYY for day granularity or an epoch for second granularity.
     * - "empty", "not_empty"
     * - "is within"
     *     - You may use an [ISO 8601 date format](https://en.wikipedia.org/wiki/ISO_8601) or the # of days.
     *
     * **Text**
     * - "contains"
     * - "eq" (is/equals - matches the full field)
     * - "ne" (is not/not equals - matches any field where the entire field is not the condition value)
     * - "empty"
     * - "not_empty"
     *
     * **Numbers**
     * - "eq" (is/equals)
     * - "lt" (is less than)
     * - "gt" (is greater than)
     * - "empty"
     * - "not_empty"
     *
     * **Email Clicks and Opens**
     * - "eq" (opened)
     * - "ne" (not opened)
     *
     * All field values must be a string.
     *
     *
     * Conditions using "eq" or "ne" for email clicks and opens should provide a "field" of either `clicks.campaign_identifier` or `opens.campaign_identifier`.
     * The condition value should be a string containing the id of a completed campaign.
     *
     *
     * The conditions list may contain multiple conditions, joined by an "and" or "or" in the "and_or" field.
     *
     * The first condition in the conditions list must have an empty "and_or", and subsequent conditions must all specify an "and_or".
     *
     * **Parameters:**
     *
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn post_contactdb_segment(
        &self,
        on_behalf_of: &str,
        body: &crate::types::ContactdbSegments,
    ) -> Result<crate::types::ContactdbSegmentsWithAllOf> {
        let url = "/contactdb/segments".to_string();
        self.client
            .post(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Retrieve a segment.
     *
     * This function performs a `GET` to the `/contactdb/segments/{segment_id}` endpoint.
     *
     * **This endpoint allows you to retrieve a single segment with the given ID.**
     *
     * **Parameters:**
     *
     * * `segment_id: i64` -- The ID of the segment you want to request.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_segments_segment(
        &self,
        segment_id: &str,
        on_behalf_of: &str,
    ) -> Result<crate::types::ContactdbSegments> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if segment_id > 0 {
            query_args.push(("segment_id".to_string(), segment_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/contactdb/segments/{}?{}",
            crate::progenitor_support::encode_path(&segment_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
     * Delete a segment.
     *
     * This function performs a `DELETE` to the `/contactdb/segments/{segment_id}` endpoint.
     *
     * **This endpoint allows you to delete a segment from your recipients database.**
     *
     * You also have the option to delete all the contacts from your Marketing Campaigns recipient database who were in this segment.
     *
     * **Parameters:**
     *
     * * `delete_contacts: bool` -- True to delete all contacts matching the segment in addition to deleting the segment.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn delete_contactdb_segments_segment(
        &self,
        segment_id: &str,
        delete_contacts: bool,
        on_behalf_of: &str,
        body: &serde_json::Value,
    ) -> Result<()> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if delete_contacts {
            query_args.push(("delete_contacts".to_string(), delete_contacts.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/contactdb/segments/{}?{}",
            crate::progenitor_support::encode_path(&segment_id.to_string()),
            query_
        );

        self.client
            .delete(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Update a segment.
     *
     * This function performs a `PATCH` to the `/contactdb/segments/{segment_id}` endpoint.
     *
     * **This endpoint allows you to update a segment.**
     *
     * **Parameters:**
     *
     * * `segment_id: &str` -- The license key provided with your New Relic account.
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn patch_contactdb_segments_segment(
        &self,
        segment_id: &str,
        on_behalf_of: &str,
        body: &crate::types::PatchContactdbSegmentsSegmentRequest,
    ) -> Result<crate::types::ContactdbSegments> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !segment_id.is_empty() {
            query_args.push(("segment_id".to_string(), segment_id.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/contactdb/segments/{}?{}",
            crate::progenitor_support::encode_path(&segment_id.to_string()),
            query_
        );

        self.client
            .patch(
                &url,
                Some(reqwest::Body::from(serde_json::to_vec(body).unwrap())),
            )
            .await
    }

    /**
     * Retrieve recipients on a segment.
     *
     * This function performs a `GET` to the `/contactdb/segments/{segment_id}/recipients` endpoint.
     *
     * **This endpoint allows you to retrieve all of the recipients in a segment with the given ID.**
     *
     * **Parameters:**
     *
     * * `page: i64`
     * * `page_size: i64`
     * * `on_behalf_of: &str` -- The license key provided with your New Relic account.
     */
    pub async fn get_contactdb_segments_segment_recipients(
        &self,
        segment_id: i64,
        page: i64,
        page_size: i64,
        on_behalf_of: &str,
    ) -> Result<crate::types::ListRecipientsOnASegmentResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if page > 0 {
            query_args.push(("page".to_string(), page.to_string()));
        }
        if page_size > 0 {
            query_args.push(("page_size".to_string(), page_size.to_string()));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/contactdb/segments/{}/recipients?{}",
            crate::progenitor_support::encode_path(&segment_id.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }
}