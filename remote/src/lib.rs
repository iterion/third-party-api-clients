//! A fully generated, opinionated API client library for Remote.
//!
//! [![docs.rs](https://docs.rs/remote-api/badge.svg)](https://docs.rs/remote-api)
//!
//! ## API Details
//!
//! # Authentication
//!
//! <!-- ReDoc-Inject: <security-definitions> -->
//!
//!
//!
//!
//!
//!
//! ## Client Details
//!
//! This client is generated from the [Remote OpenAPI
//! specs](https://remote.com/4791a4a5-62a4-4885-8408-9ea050ebf872) based on API spec version `0.1.0`. This way it will remain
//! up to date as features are added. The documentation for the crate is generated
//! along with the code to make this library easy to use.
//!
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! remote-api = "0.3.1"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```
//! use remote_api::Client;
//!
//! let remote = Client::new(
//!     String::from("client-id"),
//!     String::from("client-secret"),
//!     String::from("redirect-uri"),
//!     String::from("token"),
//!     String::from("refresh-token")
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `REMOTE_CLIENT_ID`
//! - `REMOTE_CLIENT_SECRET`
//! - `REMOTE_REDIRECT_URI`
//!
//! And then you can create a client from the environment.
//!
//! ```
//! use remote_api::Client;
//!
//! let remote = Client::new_from_env(
//!     String::from("token"),
//!     String::from("refresh-token")
//! );
//! ```
//!
//! It is okay to pass empty values for `token` and `refresh_token`. In
//! the initial state of the client, you will not know these values.
//!
//! To start off a fresh client and get a `token` and `refresh_token`, use the following.
//!
//! ```
//! use remote_api::Client;
//!
//! async fn do_call() {
//!     let mut remote = Client::new_from_env("", "");
//!
//!     // Get the URL to request consent from the user.
//!     // You can optionally pass in scopes. If none are provided, then the
//!     // resulting URL will not have any scopes.
//!     let user_consent_url = remote.user_consent_url(&["some-scope".to_string()]);
//!
//!     // In your redirect URL capture the code sent and our state.
//!     // Send it along to the request for the token.
//!     let code = "thing-from-redirect-url";
//!     let state = "state-from-redirect-url";
//!     let mut access_token = remote.get_access_token(code, state).await.unwrap();
//!
//!     // You can additionally refresh the access token with the following.
//!     // You must have a refresh token to be able to call this function.
//!     access_token = remote.refresh_access_token().await.unwrap();
//! }
//! ```
//!
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::nonstandard_macro_braces)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::tabs_in_doc_comments)]
#![allow(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod company_managers;
pub mod countries;
pub mod employments;
pub mod sandbox;
#[cfg(test)]
mod tests;
pub mod time_off;
pub mod types;
#[doc(hidden)]
pub mod utils;

use anyhow::{anyhow, Error, Result};

pub const DEFAULT_HOST: &str = "https://gateway.remote.com/";

mod progenitor_support {
    use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

    const PATH_SET: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'#')
        .add(b'<')
        .add(b'>')
        .add(b'?')
        .add(b'`')
        .add(b'{')
        .add(b'}');

    #[allow(dead_code)]
    pub(crate) fn encode_path(pc: &str) -> String {
        utf8_percent_encode(pc, PATH_SET).to_string()
    }
}

use std::convert::TryInto;
use std::env;
use std::ops::Add;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

const TOKEN_ENDPOINT: &str = "https://";
const USER_CONSENT_ENDPOINT: &str = "https://";

/// Entrypoint for interacting with the API client.
#[derive(Clone)]
pub struct Client {
    host: String,
    token: Arc<RwLock<InnerToken>>,
    client_id: String,
    client_secret: String,
    redirect_uri: String,

    auto_refresh: bool,
    client: reqwest_middleware::ClientWithMiddleware,
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, JsonSchema, Clone, Default, Serialize, Deserialize)]
pub struct AccessToken {
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub token_type: String,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub access_token: String,
    #[serde(default)]
    pub expires_in: i64,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub refresh_token: String,
    #[serde(default, alias = "x_refresh_token_expires_in")]
    pub refresh_token_expires_in: i64,

    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "crate::utils::deserialize_null_string::deserialize"
    )]
    pub scope: String,
}

/// Time in seconds before the access token expiration point that a refresh should
/// be performed. This value is subtracted from the `expires_in` value returned by
/// the provider prior to storing
const REFRESH_THRESHOLD: Duration = Duration::from_secs(60);

#[derive(Debug, Clone)]
struct InnerToken {
    access_token: String,
    refresh_token: String,
    expires_at: Option<Instant>,
}

impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    pub fn new<I, K, R, T, Q>(
        client_id: I,
        client_secret: K,
        redirect_uri: R,
        token: T,
        refresh_token: Q,
    ) -> Self
    where
        I: ToString,
        K: ToString,
        R: ToString,
        T: ToString,
        Q: ToString,
    {
        // Retry up to 3 times with increasing intervals between attempts.
        let retry_policy =
            reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
        let client = reqwest::Client::builder().build();
        match client {
            Ok(c) => {
                let client = reqwest_middleware::ClientBuilder::new(c)
                    // Trace HTTP requests. See the tracing crate to make use of these traces.
                    .with(reqwest_tracing::TracingMiddleware::default())
                    // Retry failed requests.
                    .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                        reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                        |req: &reqwest::Request| req.try_clone().is_some(),
                    ))
                    .build();

                Client {
                    host: DEFAULT_HOST.to_string(),
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                    redirect_uri: redirect_uri.to_string(),
                    token: Arc::new(RwLock::new(InnerToken {
                        access_token: token.to_string(),
                        refresh_token: refresh_token.to_string(),
                        expires_at: None,
                    })),

                    auto_refresh: false,
                    client,
                }
            }
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Enables or disables the automatic refreshing of access tokens upon expiration
    pub fn set_auto_access_token_refresh(&mut self, enabled: bool) -> &mut Self {
        self.auto_refresh = enabled;
        self
    }

    /// Sets a specific `Instant` at which the access token should be considered expired.
    /// The expiration value will only be used when automatic access token refreshing is
    /// also enabled. `None` may be passed in if the expiration is unknown. In this case
    /// automatic refreshes will be attempted when encountering an UNAUTHENTICATED status
    /// code on a response.
    pub async fn set_expires_at(&self, expires_at: Option<Instant>) -> &Self {
        self.token.write().await.expires_at = expires_at;
        self
    }

    /// Gets the `Instant` at which the access token used by this client is set to expire
    /// if one is known
    pub async fn expires_at(&self) -> Option<Instant> {
        self.token.read().await.expires_at
    }

    /// Sets the number of seconds in which the current access token should be considered
    /// expired
    pub async fn set_expires_in(&self, expires_in: i64) -> &Self {
        self.token.write().await.expires_at = Self::compute_expires_at(expires_in);
        self
    }

    /// Gets the number of seconds from now in which the current access token will be
    /// considered expired if one is known
    pub async fn expires_in(&self) -> Option<Duration> {
        self.token
            .read()
            .await
            .expires_at
            .map(|i| i.duration_since(Instant::now()))
    }

    /// Determines if the access token currently stored in the client is expired. If the
    /// expiration can not be determined, None is returned
    pub async fn is_expired(&self) -> Option<bool> {
        self.token
            .read()
            .await
            .expires_at
            .map(|expiration| expiration <= Instant::now())
    }

    fn compute_expires_at(expires_in: i64) -> Option<Instant> {
        let seconds_valid = expires_in
            .try_into()
            .ok()
            .map(Duration::from_secs)
            .and_then(|dur| dur.checked_sub(REFRESH_THRESHOLD))
            .or_else(|| Some(Duration::from_secs(0)));

        seconds_valid.map(|seconds_valid| Instant::now().add(seconds_valid))
    }

    /// Override the default host for the client.
    pub fn with_host<H>(&self, host: H) -> Self
    where
        H: ToString,
    {
        let mut c = self.clone();
        c.host = host.to_string();
        c
    }

    /// Create a new Client struct from environment variables. It
    /// takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key and your requests will work.
    /// We pass in the token and refresh token to the client so if you are storing
    /// it in a database, you can get it first.
    pub fn new_from_env<T, R>(token: T, refresh_token: R) -> Self
    where
        T: ToString,
        R: ToString,
    {
        let client_id = env::var("REMOTE_CLIENT_ID").expect("must set REMOTE_CLIENT_ID");
        let client_secret =
            env::var("REMOTE_CLIENT_SECRET").expect("must set REMOTE_CLIENT_SECRET");
        let redirect_uri = env::var("REMOTE_REDIRECT_URI").expect("must set REMOTE_REDIRECT_URI");

        Client::new(client_id, client_secret, redirect_uri, token, refresh_token)
    }

    /// Return a user consent url with an optional set of scopes.
    /// If no scopes are provided, they will not be passed in the url.
    pub fn user_consent_url(&self, scopes: &[String]) -> String {
        let state = uuid::Uuid::new_v4();

        let url = format!(
            "{}?client_id={}&response_type=code&redirect_uri={}&state={}",
            USER_CONSENT_ENDPOINT, self.client_id, self.redirect_uri, state
        );

        if scopes.is_empty() {
            return url;
        }

        // Add the scopes.
        format!("{}&scope={}", url, scopes.join(" "))
    }

    /// Refresh an access token from a refresh token. Client must have a refresh token
    /// for this to work.
    pub async fn refresh_access_token(&self) -> Result<AccessToken> {
        let response = {
            let refresh_token = &self.token.read().await.refresh_token;

            if refresh_token.is_empty() {
                anyhow!("refresh token cannot be empty");
            }

            let mut headers = reqwest::header::HeaderMap::new();
            headers.append(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            );

            let params = [
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("redirect_uri", &self.redirect_uri),
            ];
            let client = reqwest::Client::new();
            client
                .post(TOKEN_ENDPOINT)
                .headers(headers)
                .form(&params)
                .basic_auth(&self.client_id, Some(&self.client_secret))
                .send()
                .await?
        };

        // Unwrap the response.
        let t: AccessToken = response.json().await?;

        let refresh_token = self.token.read().await.refresh_token.clone();

        *self.token.write().await = InnerToken {
            access_token: t.access_token.clone(),
            refresh_token,
            expires_at: Self::compute_expires_at(t.expires_in),
        };

        Ok(t)
    }

    /// Get an access token from the code returned by the URL paramter sent to the
    /// redirect URL.
    pub async fn get_access_token(&mut self, code: &str, state: &str) -> Result<AccessToken> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("redirect_uri", &self.redirect_uri),
            ("state", state),
        ];
        let client = reqwest::Client::new();
        let resp = client
            .post(TOKEN_ENDPOINT)
            .headers(headers)
            .form(&params)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await?;

        // Unwrap the response.
        let t: AccessToken = resp.json().await?;

        *self.token.write().await = InnerToken {
            access_token: t.access_token.clone(),
            refresh_token: t.refresh_token.clone(),
            expires_at: Self::compute_expires_at(t.expires_in),
        };

        Ok(t)
    }

    async fn url_and_auth(&self, uri: &str) -> Result<(reqwest::Url, Option<String>)> {
        let parsed_url = uri.parse::<reqwest::Url>();
        let auth = format!("Bearer {}", self.token);
        parsed_url.map(|u| (u, Some(auth))).map_err(Error::from)
    }

    async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<reqwest::Response> {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;
        let instance = <&Client>::clone(&self);
        let mut req = instance.client.request(method.clone(), url);
        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }
        if let Some(body) = body {
            req = req.body(body);
        }
        Ok(req.send().await?)
    }

    async fn request<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.request_raw(method, uri, body).await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };

            Err(error)
        }
    }

    async fn request_with_links<Out>(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<(Option<hyperx::header::Link>, Out)>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let response = self.request_raw(method, uri, body).await?;

        let status = response.status();
        let link = response
            .headers()
            .get(http::header::LINK)
            .and_then(|l| l.to_str().ok())
            .and_then(|l| l.parse().ok());

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");

            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map(|out| (link, out)).map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };
            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn post_form<Out>(&self, uri: &str, form: reqwest::multipart::Form) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(http::Method::POST, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        req = req.multipart(form);

        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {
                // Parse the output as a string.
                serde_json::from_value(serde_json::json!(&String::from_utf8(
                    response_body.to_vec()
                )?))
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };

            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn request_with_accept_mime<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        accept_mime_type: &str,
    ) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(method, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_str(accept_mime_type)?,
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else if std::any::TypeId::of::<Out>() == std::any::TypeId::of::<String>() {
                // Parse the output as a string.
                serde_json::from_value(serde_json::json!(&String::from_utf8(
                    response_body.to_vec()
                )?))
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };

            Err(error)
        }
    }

    /* TODO: make this more DRY */
    #[allow(dead_code)]
    async fn request_with_mime<Out>(
        &self,
        method: reqwest::Method,
        uri: &str,
        content: &[u8],
        mime_type: &str,
    ) -> Result<Out>
    where
        Out: serde::de::DeserializeOwned + 'static + Send,
    {
        let u = if uri.starts_with("https://") {
            uri.to_string()
        } else {
            (self.host.clone() + uri).to_string()
        };
        let (url, auth) = self.url_and_auth(&u).await?;

        let instance = <&Client>::clone(&self);

        let mut req = instance.client.request(method, url);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_bytes(mime_type.as_bytes()).unwrap(),
        );
        // We are likely uploading a file so add the right headers.
        req = req.header(
            reqwest::header::HeaderName::from_static("x-upload-content-type"),
            reqwest::header::HeaderValue::from_static("application/octet-stream"),
        );
        req = req.header(
            reqwest::header::HeaderName::from_static("x-upload-content-length"),
            reqwest::header::HeaderValue::from_bytes(format!("{}", content.len()).as_bytes())
                .unwrap(),
        );

        if let Some(auth_str) = auth {
            req = req.header(http::header::AUTHORIZATION, &*auth_str);
        }

        if content.len() > 1 {
            let b = bytes::Bytes::copy_from_slice(content);
            // We are uploading a file so add that as the body.
            req = req.body(b);
        }

        let response = req.send().await?;

        let status = response.status();

        let response_body = response.bytes().await?;

        if status.is_success() {
            log::debug!("Received successful response. Read payload.");
            let parsed_response = if status == http::StatusCode::NO_CONTENT
                || std::any::TypeId::of::<Out>() == std::any::TypeId::of::<()>()
            {
                serde_json::from_str("null")
            } else {
                serde_json::from_slice::<Out>(&response_body)
            };
            parsed_response.map_err(Error::from)
        } else {
            let error = if response_body.is_empty() {
                anyhow!("code: {}, empty response", status)
            } else {
                anyhow!(
                    "code: {}, error: {:?}",
                    status,
                    String::from_utf8_lossy(&response_body),
                )
            };

            Err(error)
        }
    }

    async fn request_entity<D>(
        &self,
        method: http::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let r = self.request(method, uri, body).await?;
        Ok(r)
    }

    #[allow(dead_code)]
    async fn get<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::GET, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn get_all_pages<D>(&self, uri: &str, _message: Option<reqwest::Body>) -> Result<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        // TODO: implement this.
        self.unfold(uri).await
    }

    /// "unfold" paginated results of a vector of items
    #[allow(dead_code)]
    async fn unfold<D>(&self, uri: &str) -> Result<Vec<D>>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        let mut global_items = Vec::new();
        let (new_link, mut items) = self.get_pages(uri).await?;
        let mut link = new_link;
        while !items.is_empty() {
            global_items.append(&mut items);
            // We need to get the next link.
            if let Some(url) = link.as_ref().and_then(crate::utils::next_link) {
                let url = reqwest::Url::parse(&url)?;
                let (new_link, new_items) = self.get_pages_url(&url).await?;
                link = new_link;
                items = new_items;
            }
        }

        Ok(global_items)
    }

    #[allow(dead_code)]
    async fn get_pages<D>(&self, uri: &str) -> Result<(Option<hyperx::header::Link>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_with_links(http::Method::GET, &(self.host.to_string() + uri), None)
            .await
    }

    #[allow(dead_code)]
    async fn get_pages_url<D>(
        &self,
        url: &reqwest::Url,
    ) -> Result<(Option<hyperx::header::Link>, Vec<D>)>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_with_links(http::Method::GET, url.as_str(), None)
            .await
    }

    #[allow(dead_code)]
    async fn post<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::POST, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn patch<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PATCH, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn put<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(http::Method::PUT, &(self.host.to_string() + uri), message)
            .await
    }

    #[allow(dead_code)]
    async fn delete<D>(&self, uri: &str, message: Option<reqwest::Body>) -> Result<D>
    where
        D: serde::de::DeserializeOwned + 'static + Send,
    {
        self.request_entity(
            http::Method::DELETE,
            &(self.host.to_string() + uri),
            message,
        )
        .await
    }

    pub fn company_managers(&self) -> company_managers::CompanyManagers {
        company_managers::CompanyManagers::new(self.clone())
    }

    pub fn countries(&self) -> countries::Countries {
        countries::Countries::new(self.clone())
    }

    pub fn employments(&self) -> employments::Employments {
        employments::Employments::new(self.clone())
    }

    pub fn sandbox(&self) -> sandbox::Sandbox {
        sandbox::Sandbox::new(self.clone())
    }

    pub fn time_off(&self) -> time_off::TimeOff {
        time_off::TimeOff::new(self.clone())
    }
}
