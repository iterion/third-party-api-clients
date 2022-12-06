use anyhow::Result;

use crate::Client;

pub struct Countries {
    pub client: Client,
}

impl Countries {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Countries { client }
    }

    /**
    * List countries.
    *
    * This function performs a `GET` to the `/v1/countries` endpoint.
    *
    * Returns a list of all countries that are supported by Remote API alphabetically ordered.
    */
    pub async fn get_supported_country(&self) -> Result<crate::types::CountriesResponse> {
        let url = "/v1/countries".to_string();
        self.client.get(&url, None).await
    }

    /**
    * List all holidays of a country.
    *
    * This function performs a `GET` to the `/v1/countries/{country_code}/holidays/{year}` endpoint.
    *
    * List all holidays of a country for a specific year. Optionally, it can be filtered by country subdivision.
    *
    * **Parameters:**
    *
    * * `country_code: &str` -- Country code according to ISO 3166-1 3-digit alphabetic codes.
    * * `year: &str` -- Year for the holidays.
    * * `country_subdivision_code: &str` -- Country subdivision code according to ISO 3166-2 codes.
    */
    pub async fn get_index_holiday(
        &self,
        country_code: &str,
        year: &str,
        country_subdivision_code: &str,
    ) -> Result<crate::types::HolidaysResponse> {
        let mut query_args: Vec<(String, String)> = Default::default();
        if !country_subdivision_code.is_empty() {
            query_args.push((
                "country_subdivision_code".to_string(),
                country_subdivision_code.to_string(),
            ));
        }
        let query_ = serde_urlencoded::to_string(&query_args).unwrap();
        let url = format!(
            "/v1/countries/{}/holidays/{}?{}",
            crate::progenitor_support::encode_path(&country_code.to_string()),
            crate::progenitor_support::encode_path(&year.to_string()),
            query_
        );

        self.client.get(&url, None).await
    }

    /**
    * Show form schema.
    *
    * This function performs a `GET` to the `/v1/countries/{country_code}/{form}` endpoint.
    *
    * Returns the json schema of a supported form. Possible form names are:
    *
    *     - address_details
    *     - administrative_details
    *     - bank_account_details
    *     - billing_address_details
    *     - contract_details
    *     - emergency_contact_details
    *     - employment_document_details
    *     - personal_details
    *     - pricing_plan_details
    *
    *     
    *
    * **Parameters:**
    *
    * * `country_code: &str` -- Country code according to ISO 3-digit alphabetic codes.
    * * `form: &str` -- Name of the desired form.
    */
    pub async fn get_show_form_country(
        &self,
        country_code: &str,
        form: &str,
    ) -> Result<crate::types::CountryFormResponse> {
        let url = format!(
            "/v1/countries/{}/{}",
            crate::progenitor_support::encode_path(&country_code.to_string()),
            crate::progenitor_support::encode_path(&form.to_string()),
        );

        self.client.get(&url, None).await
    }
}
