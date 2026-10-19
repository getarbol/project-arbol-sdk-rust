//! The Arbol API over HTTPS, for Rust.
//!
//! ```no_run
//! # async fn run() -> Result<(), arbol_sdk::Error> {
//! use arbol_sdk::{Client, Credential, types};
//!
//! let arbol = Client::new(Credential::ApiKey(std::env::var("ARBOL_API_KEY").unwrap()))?;
//! let input = types::AgendaAppointmentsListInput {
//!     from: Some("2026-10-12T00:00:00Z".into()),
//!     to: Some("2026-10-19T00:00:00Z".into()),
//!     ..Default::default()
//! };
//! let page = arbol.agenda().appointments().list(&input).await?;
//! # Ok(())
//! # }
//! ```
//!
//! Every operation, type and route is generated from the platform's public OpenAPI by Cloudflare Forge
//! (`src/generated`, never edited by hand); this file is the transport, the same as `@arbol-ai/sdk`'s:
//! exactly one credential, HTTPS only, no redirect followed, a response bounded by its operation's
//! limit, and nothing ever retried. A failed write says that its outcome is unknown; reconcile it with
//! its owner before repeating it.

use std::time::Duration;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

// Forge's output, kept exactly as the plugin writes it.
#[rustfmt::skip]
mod generated;

pub use generated::client::*;
pub use generated::{operations, types};

/// The release these operations and types come from.
pub use generated::operations::RELEASE;

/// How a query field is written: the representation the Gateway reads back for its declared kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryKind {
    String,
    Integer,
    Number,
    Boolean,
    List,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effect {
    Read,
    Write,
    Delete,
}

/// How one published operation is reached. Only generated descriptors supply routes.
#[derive(Debug, Clone, Copy)]
pub struct Operation {
    pub name: &'static str,
    pub method: &'static str,
    pub path: &'static str,
    pub path_field: Option<&'static str>,
    /// GET and DELETE: every other input field and how it is written; body methods have none.
    pub query: Option<&'static [(&'static str, QueryKind)]>,
    pub effect: Effect,
    pub max_output_bytes: usize,
}

/// What is known about a failed call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// Nothing happened.
    Refused,
    /// Stopped before the request left.
    NotSent,
    /// A write may have reached its owner; reconcile before repeating it.
    Unknown,
}

/// Every failure of an Arbol call. Nothing is ever retried by this client.
#[derive(Debug, thiserror::Error)]
#[error("{code}: {message}")]
pub struct Error {
    pub code: String,
    pub message: String,
    pub status: Option<u16>,
    pub retryable: bool,
    pub outcome: Outcome,
    pub request_id: Option<String>,
    pub data: Value,
}

impl Error {
    fn new(code: &str, message: impl Into<String>, outcome: Outcome) -> Self {
        Self {
            code: code.to_owned(),
            message: message.into(),
            status: None,
            retryable: false,
            outcome,
            request_id: None,
            data: Value::Null,
        }
    }
    fn not_sent(message: impl Into<String>) -> Self {
        Self::new("INVALID_INPUT", message, Outcome::NotSent)
    }
}

/// Exactly one credential: an API key (`arb_live_…`, a machine) or a person's access token for the
/// audience `https://api.getarbol.com`.
#[derive(Clone)]
pub enum Credential {
    ApiKey(String),
    AccessToken(String),
}

impl std::fmt::Debug for Credential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::ApiKey(_) => "ApiKey(…)",
            Self::AccessToken(_) => "AccessToken(…)",
        })
    }
}

/// The Arbol API over HTTPS.
#[derive(Debug, Clone)]
pub struct Client {
    base: reqwest::Url,
    credential: Credential,
    http: reqwest::Client,
}

impl Client {
    /// A client of `https://api.getarbol.com`.
    pub fn new(credential: Credential) -> Result<Self, Error> {
        Self::with_base_url(credential, "https://api.getarbol.com")
    }

    /// A client of another trusted deployment, by its HTTPS origin.
    pub fn with_base_url(credential: Credential, base_url: &str) -> Result<Self, Error> {
        Self::build(credential, base_url, "https")
    }

    /// Only the tests reach a loopback server over plain HTTP.
    fn build(credential: Credential, base_url: &str, scheme: &str) -> Result<Self, Error> {
        let base = reqwest::Url::parse(base_url).map_err(|_| Error::not_sent("base_url must be an HTTPS origin"))?;
        if base.scheme() != scheme
            || !base.username().is_empty()
            || base.password().is_some()
            || base.path() != "/"
            || base.query().is_some()
            || base.fragment().is_some()
        {
            return Err(Error::not_sent("base_url must be an HTTPS origin"));
        }
        let http = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(|_| Error::not_sent("Could not build the HTTP client"))?;
        Ok(Self { base, credential, http })
    }

    /// Call one operation with its input, returning the data of its success envelope.
    pub async fn call<I: Serialize + ?Sized, O: DeserializeOwned>(
        &self,
        op: &Operation,
        input: &I,
    ) -> Result<O, Error> {
        let mut remaining = match serde_json::to_value(input) {
            Ok(Value::Object(map)) => map,
            _ => return Err(Error::not_sent("Input must be an object")),
        };
        let mut path = op.path.to_owned();
        if let Some(field) = op.path_field {
            let value = match remaining.remove(field) {
                Some(Value::String(value)) if valid_segment(&value) => value,
                _ => return Err(Error::not_sent(format!("Invalid {field}"))),
            };
            path = path.replace(&format!("{{{field}}}"), &encode_segment(&value));
        }
        if !path.starts_with("/v1/") {
            return Err(Error::not_sent("Invalid operation route"));
        }
        let mut url = self
            .base
            .join(&path)
            .map_err(|_| Error::not_sent("Invalid operation route"))?;
        if url.origin() != self.base.origin() {
            return Err(Error::not_sent("Invalid operation route"));
        }
        let token = match &self.credential {
            Credential::ApiKey(token) | Credential::AccessToken(token) => token,
        };
        if token.is_empty() || token.chars().any(char::is_whitespace) {
            return Err(Error::new(
                "UNAUTHORIZED",
                "Missing or invalid credential",
                Outcome::NotSent,
            ));
        }
        let method =
            reqwest::Method::from_bytes(op.method.as_bytes()).map_err(|_| Error::not_sent("Invalid method"))?;
        let mut request;
        if let Some(kinds) = op.query {
            {
                let mut pairs = url.query_pairs_mut();
                // A field left out is absent from the map; null cannot be written in a query.
                for (field, value) in &remaining {
                    let kind = kinds
                        .iter()
                        .find(|(name, _)| name == field)
                        .map(|(_, kind)| *kind)
                        .ok_or_else(|| Error::not_sent(format!("Unknown field {field}")))?;
                    pairs.append_pair(field, &query_value(field, kind, value)?);
                }
            }
            if url.query() == Some("") {
                url.set_query(None);
            }
            request = self.http.request(method, url);
        } else {
            request = self.http.request(method, url).json(&Value::Object(remaining));
        }
        request = request.header("accept", "application/json").bearer_auth(token);
        let unknown = if op.effect == Effect::Read {
            Outcome::Refused
        } else {
            Outcome::Unknown
        };
        let mut response = request
            .send()
            .await
            .map_err(|_| Error::new("NETWORK_ERROR", "The request failed and was not retried", unknown))?;
        let status = response.status().as_u16();
        let request_id = response
            .headers()
            .get("x-request-id")
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned);
        let fail = |code: &str, message: &str| Error {
            status: Some(status),
            request_id: request_id.clone(),
            ..Error::new(code, message, unknown)
        };
        if (300..400).contains(&status) {
            return Err(fail(
                "REDIRECT_REFUSED",
                "A redirect was refused and the request was not retried",
            ));
        }
        let invalid = || {
            fail(
                "INVALID_RESPONSE",
                "The response was invalid or incomplete; the request was not retried",
            )
        };
        let json = response
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok())
            .is_some_and(|value| value.split(';').next() == Some("application/json"));
        let mut body = Vec::new();
        while let Some(chunk) = response.chunk().await.map_err(|_| invalid())? {
            if body.len() + chunk.len() > op.max_output_bytes {
                return Err(invalid());
            }
            body.extend_from_slice(&chunk);
        }
        if !json {
            return Err(invalid());
        }
        let envelope: Map<String, Value> = match serde_json::from_slice(&body) {
            Ok(Value::Object(map)) => map,
            _ => return Err(invalid()),
        };
        if envelope.get("success") == Some(&Value::Bool(true)) {
            let data = envelope.get("data").cloned().unwrap_or(Value::Null);
            return serde_json::from_value(data).map_err(|_| invalid());
        }
        let error = envelope.get("error").and_then(Value::as_object);
        let code = error
            .and_then(|e| e.get("code"))
            .and_then(Value::as_str)
            .unwrap_or("INVALID_RESPONSE")
            .to_owned();
        // A write may have happened when its owner says so, or when the Gateway or the owner could not
        // complete or answer the call.
        let outcome = match code.as_str() {
            "OUTCOME_UNKNOWN" | "UPSTREAM_UNAVAILABLE" | "UPSTREAM_TIMEOUT" | "INTERNAL_ERROR" | "INVALID_RESPONSE" => {
                unknown
            }
            _ => Outcome::Refused,
        };
        Err(Error {
            message: error
                .and_then(|e| e.get("message"))
                .and_then(Value::as_str)
                .unwrap_or("The operation failed")
                .to_owned(),
            status: Some(status),
            retryable: error.and_then(|e| e.get("retryable")) == Some(&Value::Bool(true)),
            outcome,
            request_id,
            data: error.and_then(|e| e.get("data")).cloned().unwrap_or(Value::Null),
            code,
        })
    }
}

/// Whether a path value stays in its segment: no control character, space, `/`, `\`, `?` or `#`.
fn valid_segment(value: &str) -> bool {
    !value.is_empty()
        && value != "."
        && value != ".."
        && !value.chars().any(|c| (c as u32) <= 0x20 || "/\\?#".contains(c))
}

/// `encodeURIComponent`, as the TypeScript SDK writes a path value.
fn encode_segment(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&byte) {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

fn query_value(field: &str, kind: QueryKind, value: &Value) -> Result<String, Error> {
    let wrong = || Error::not_sent(format!("{field} must be a {}", format!("{kind:?}").to_lowercase()));
    match (kind, value) {
        (QueryKind::List, Value::Array(items)) => {
            let strings: Option<Vec<&str>> = items
                .iter()
                .map(|item| item.as_str().filter(|s| !s.is_empty() && !s.contains(',')))
                .collect();
            match strings {
                Some(strings) if !strings.is_empty() => Ok(strings.join(",")),
                _ => Err(Error::not_sent(format!(
                    "{field} must be a non-empty list of strings without commas"
                ))),
            }
        }
        (QueryKind::String, Value::String(s)) => Ok(s.clone()),
        (QueryKind::Integer | QueryKind::Number, Value::Number(n)) => Ok(n.to_string()),
        (QueryKind::Boolean, Value::Bool(b)) => Ok(b.to_string()),
        _ => Err(wrong()),
    }
}

/// A field that may be absent, null or a value: `None` leaves it out, `Some(None)` sends `null`.
pub mod nullable {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<T: Serialize, S: Serializer>(value: &Option<Option<T>>, serializer: S) -> Result<S::Ok, S::Error> {
        match value {
            Some(inner) => inner.serialize(serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, T: Deserialize<'de>, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Option<T>>, D::Error> {
        Option::<T>::deserialize(deserializer).map(Some)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    /// A loopback server that answers each request with `response` and records what it received.
    async fn server(response: &'static str) -> (String, tokio::task::JoinHandle<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let origin = format!("http://{}", listener.local_addr().unwrap());
        let handle = tokio::spawn(async move {
            let mut requests = Vec::new();
            while let Ok(Ok((mut socket, _))) =
                tokio::time::timeout(Duration::from_millis(300), listener.accept()).await
            {
                let mut buffer = vec![0; 65536];
                let n = socket.read(&mut buffer).await.unwrap();
                requests.push(String::from_utf8_lossy(&buffer[..n]).into_owned());
                socket.write_all(response.as_bytes()).await.unwrap();
                socket.shutdown().await.ok();
            }
            requests
        });
        (origin, handle)
    }

    fn client(origin: &str) -> Client {
        Client::build(Credential::ApiKey("arb_live_synthetic".into()), origin, "http").unwrap()
    }

    const LIST: Operation = Operation {
        name: "agenda.appointments.list",
        method: "GET",
        path: "/v1/agenda/appointments/{id}",
        path_field: Some("id"),
        query: Some(&[("status", QueryKind::List), ("limit", QueryKind::Integer)]),
        effect: Effect::Read,
        max_output_bytes: 1024,
    };
    const CREATE: Operation = Operation {
        name: "agenda.appointments.create",
        method: "POST",
        path: "/v1/agenda/appointments",
        path_field: None,
        query: None,
        effect: Effect::Write,
        max_output_bytes: 1024,
    };

    fn ok(body: &str) -> String {
        format!(
            "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\nx-request-id: req_1\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
            body.len()
        )
    }

    #[tokio::test]
    async fn sends_path_and_query_and_returns_data() {
        let response: &'static str = Box::leak(ok(r#"{"success":true,"data":{"n":1},"error":null}"#).into_boxed_str());
        let (origin, handle) = server(response).await;
        let input = serde_json::json!({"id": "ab:c%é", "status": ["booked", "done"], "limit": 5});
        let data: Value = client(&origin).call(&LIST, &input).await.unwrap();
        assert_eq!(data, serde_json::json!({"n": 1}));
        let requests = handle.await.unwrap();
        assert_eq!(requests.len(), 1);
        assert!(
            requests[0].starts_with("GET /v1/agenda/appointments/ab%3Ac%25%C3%A9?"),
            "{}",
            requests[0]
        );
        assert!(requests[0].contains("status=booked%2Cdone"));
        assert!(requests[0].contains("limit=5"));
        assert!(
            requests[0]
                .to_lowercase()
                .contains("authorization: bearer arb_live_synthetic")
        );
    }

    #[tokio::test]
    async fn a_failed_write_has_an_unknown_outcome_and_is_never_retried() {
        let response: &'static str = Box::leak(
            ok(r#"{"success":false,"data":null,"error":{"code":"UPSTREAM_UNAVAILABLE","message":"down","retryable":true}}"#)
                .replace("200 OK", "503 Service Unavailable")
                .into_boxed_str(),
        );
        let (origin, handle) = server(response).await;
        let error = client(&origin)
            .call::<_, Value>(&CREATE, &serde_json::json!({}))
            .await
            .unwrap_err();
        assert_eq!(error.code, "UPSTREAM_UNAVAILABLE");
        assert_eq!(error.outcome, Outcome::Unknown);
        assert_eq!(error.request_id.as_deref(), Some("req_1"));
        assert_eq!(handle.await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn refuses_redirects_and_oversized_answers() {
        let (origin, handle) =
            server("HTTP/1.1 302 Found\r\nlocation: https://elsewhere.invalid/\r\ncontent-length: 0\r\nconnection: close\r\n\r\n").await;
        let error = client(&origin)
            .call::<_, Value>(&CREATE, &serde_json::json!({}))
            .await
            .unwrap_err();
        assert_eq!(error.code, "REDIRECT_REFUSED");
        assert_eq!(handle.await.unwrap().len(), 1);
        let big: &'static str =
            Box::leak(ok(&format!(r#"{{"success":true,"data":"{}"}}"#, "x".repeat(2048))).into_boxed_str());
        let (origin, _) = server(big).await;
        let error = client(&origin)
            .call::<_, Value>(&LIST, &serde_json::json!({"id": "1"}))
            .await
            .unwrap_err();
        assert_eq!(error.code, "INVALID_RESPONSE");
    }

    #[tokio::test]
    async fn stops_an_invalid_input_before_sending() {
        let error = client("http://127.0.0.1:9")
            .call::<_, Value>(&LIST, &serde_json::json!({"id": "../x"}))
            .await
            .unwrap_err();
        assert_eq!(error.outcome, Outcome::NotSent);
        assert!(Client::with_base_url(Credential::ApiKey("k".into()), "http://api.getarbol.com").is_err());
    }

    #[test]
    fn nullable_fields_tell_absent_from_null() {
        #[derive(serde::Serialize, serde::Deserialize, Default)]
        struct Patch {
            #[serde(default, skip_serializing_if = "Option::is_none", with = "crate::nullable")]
            label: Option<Option<String>>,
        }
        assert_eq!(serde_json::to_string(&Patch::default()).unwrap(), "{}");
        assert_eq!(
            serde_json::to_string(&Patch { label: Some(None) }).unwrap(),
            r#"{"label":null}"#
        );
        let read: Patch = serde_json::from_str(r#"{"label":null}"#).unwrap();
        assert_eq!(read.label, Some(None));
        let read: Patch = serde_json::from_str("{}").unwrap();
        assert_eq!(read.label, None);
    }
}
