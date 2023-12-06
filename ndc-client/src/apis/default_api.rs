use opentelemetry::{
    global,
    trace::{FutureExt, Tracer},
    Context,
};
use reqwest::{self, RequestBuilder};
use serde_json as json;
use std::collections::HashMap;

use self::utils::FutureTracing;

use super::{configuration, ConnectorURLError, Error};

trait ToHeaderString {
    fn to_header_string(self) -> String;
}

impl ToHeaderString for HashMap<String, json::Value> {
    fn to_header_string(self) -> String {
        json::to_value(self).map_or("".to_string(), |val| val.to_string())
    }
}

fn inject_trace_context(builder: RequestBuilder) -> RequestBuilder {
    let ctx = Context::current();
    let mut trace_headers = HashMap::new();
    global::get_text_map_propagator(|propagator| {
        propagator.inject_context(&ctx, &mut trace_headers);
    });
    let mut req_builder = builder;
    for (key, value) in trace_headers {
        req_builder = req_builder.header(key, value);
    }
    req_builder
}

fn append_path(url: &reqwest::Url, path: &str) -> Result<reqwest::Url, ConnectorURLError> {
    if url.path_segments().map_or(false, |mut s|
        // It is safe to unwrap here as according to documentation of Url::path_segments()
        // > When Some is returned, the iterator always contains at least one string (which may be empty).
        s.next_back().unwrap() != "")
    {
        let mut url = url.clone();
        // No trailing slash, add it
        url.path_segments_mut()
            .map_err(|_| ConnectorURLError::URLCannotBeABase())?
            .push("");
        url.join(path).map_err(ConnectorURLError::URLParseError)
    } else {
        url.join(path).map_err(ConnectorURLError::URLParseError)
    }
}

impl ToHeaderString for &str {
    fn to_header_string(self) -> String {
        self.to_string()
    }
}

pub async fn capabilities_get(
    configuration: &configuration::Configuration,
) -> Result<crate::models::CapabilitiesResponse, Error> {
    let tracer = global::tracer("engine");
    tracer
        .in_span("capabilities_get", |ctx| async {
            let configuration = configuration;

            let client = &configuration.client;

            let uri = append_path(&configuration.base_path, "capabilities")
                .map_err(Error::ConnectorURLError)?;
            let mut req_builder = client.request(reqwest::Method::GET, uri);

            req_builder = inject_trace_context(req_builder);

            if let Some(ref user_agent) = configuration.user_agent {
                req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
            }

            // Note: The headers will be merged in to any already set.
            req_builder = req_builder.headers(configuration.headers.clone());

            let req = req_builder.build()?;
            let resp = client.execute(req).with_traced_errors().await?;

            let response_status = resp.status();
            let response_content = resp.json().with_traced_errors().with_context(ctx).await?;

            if !response_status.is_client_error() && !response_status.is_server_error() {
                serde_json::from_value(response_content).map_err(Error::from)
            } else {
                let error_response: crate::models::ErrorResponse =
                    serde_json::from_value(response_content)?;
                let connector_error = super::ConnectorError {
                    status: response_status,
                    error_response,
                };
                Err(Error::ConnectorError(connector_error))
            }
        })
        .await
}

pub async fn explain_post(
    configuration: &configuration::Configuration,
    query_request: crate::models::QueryRequest,
) -> Result<crate::models::ExplainResponse, Error> {
    let tracer = global::tracer("engine");
    tracer
        .in_span("explain_post", |ctx| async {
            let configuration = configuration;

            let client = &configuration.client;

            let uri = append_path(&configuration.base_path, "explain")
                .map_err(Error::ConnectorURLError)?;
            let mut req_builder = client.request(reqwest::Method::POST, uri);

            if let Some(ref user_agent) = configuration.user_agent {
                req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
            }

            // Note: The headers will be merged in to any already set.
            req_builder = req_builder.headers(configuration.headers.clone());

            req_builder = req_builder.json(&query_request);

            req_builder = inject_trace_context(req_builder);

            let req = req_builder.build()?;
            let resp = client.execute(req).with_traced_errors().await?;

            let response_status = resp.status();
            let response_content = resp.json().with_traced_errors().with_context(ctx).await?;

            if !response_status.is_client_error() && !response_status.is_server_error() {
                serde_json::from_value(response_content).map_err(Error::from)
            } else {
                let error_response: crate::models::ErrorResponse =
                    serde_json::from_value(response_content)?;
                let connector_error = super::ConnectorError {
                    status: response_status,
                    error_response,
                };
                Err(Error::ConnectorError(connector_error))
            }
        })
        .await
}

pub async fn mutation_post(
    configuration: &configuration::Configuration,
    mutation_request: crate::models::MutationRequest,
) -> Result<crate::models::MutationResponse, Error> {
    let tracer = global::tracer("engine");
    tracer
        .in_span("mutation_post", |ctx| async {
            let configuration = configuration;

            let client = &configuration.client;

            let uri = append_path(&configuration.base_path, "mutation")
                .map_err(Error::ConnectorURLError)?;
            let mut req_builder = client.request(reqwest::Method::POST, uri);

            if let Some(ref user_agent) = configuration.user_agent {
                req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
            }

            // Note: The headers will be merged in to any already set.
            req_builder = req_builder.headers(configuration.headers.clone());

            req_builder = req_builder.json(&mutation_request);

            req_builder = inject_trace_context(req_builder);

            let req = req_builder.build()?;
            let resp = client.execute(req).with_traced_errors().await?;

            let response_status = resp.status();
            let response_content = resp.json().with_traced_errors().with_context(ctx).await?;

            if !response_status.is_client_error() && !response_status.is_server_error() {
                serde_json::from_value(response_content).map_err(Error::from)
            } else {
                let error_response: crate::models::ErrorResponse =
                    serde_json::from_value(response_content)?;
                let connector_error = super::ConnectorError {
                    status: response_status,
                    error_response,
                };
                Err(Error::ConnectorError(connector_error))
            }
        })
        .await
}

pub async fn query_post(
    configuration: &configuration::Configuration,
    query_request: crate::models::QueryRequest,
) -> Result<crate::models::QueryResponse, Error> {
    let tracer = global::tracer("engine");
    tracer
        .in_span("query_post", |ctx| {
            async {
                let configuration = configuration;

                let client = &configuration.client;

                let uri = append_path(&configuration.base_path, "query")
                    .map_err(Error::ConnectorURLError)?;
                let mut req_builder = client.request(reqwest::Method::POST, uri);

                if let Some(ref user_agent) = configuration.user_agent {
                    req_builder =
                        req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
                }

                // Note: The headers will be merged in to any already set.
                req_builder = req_builder.headers(configuration.headers.clone());

                req_builder = req_builder.json(&query_request);

                req_builder = inject_trace_context(req_builder);

                let req = req_builder.build()?;
                let resp = client.execute(req).with_traced_errors().await?;

                let response_status = resp.status();
                let response_content = resp.json().with_traced_errors().await?;

                if !response_status.is_client_error() && !response_status.is_server_error() {
                    serde_json::from_value(response_content).map_err(Error::from)
                } else {
                    let error_response: crate::models::ErrorResponse =
                        serde_json::from_value(response_content)?;
                    let connector_error = super::ConnectorError {
                        status: response_status,
                        error_response,
                    };
                    Err(Error::ConnectorError(connector_error))
                }
            }
            .with_context(ctx)
        })
        .await
}

pub async fn schema_get(
    configuration: &configuration::Configuration,
) -> Result<crate::models::SchemaResponse, Error> {
    let tracer = global::tracer("engine");
    tracer
        .in_span("schema_get", |ctx| async {
            let configuration = configuration;

            let client = &configuration.client;

            let uri = append_path(&configuration.base_path, "schema")
                .map_err(Error::ConnectorURLError)?;
            let mut req_builder = client.request(reqwest::Method::GET, uri);

            req_builder = inject_trace_context(req_builder);

            if let Some(ref user_agent) = configuration.user_agent {
                req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
            }

            // Note: The headers will be merged in to any already set.
            req_builder = req_builder.headers(configuration.headers.clone());

            let req = req_builder.build()?;
            let resp = client.execute(req).with_traced_errors().await?;

            let response_status = resp.status();
            let response_content = resp.json().with_traced_errors().with_context(ctx).await?;

            if !response_status.is_client_error() && !response_status.is_server_error() {
                serde_json::from_value(response_content).map_err(Error::from)
            } else {
                let error_response: crate::models::ErrorResponse =
                    serde_json::from_value(response_content)?;
                let connector_error = super::ConnectorError {
                    status: response_status,
                    error_response,
                };
                Err(Error::ConnectorError(connector_error))
            }
        })
        .await
}

mod utils {
    use async_trait::async_trait;
    use opentelemetry::trace::get_active_span;
    use std::{fmt::Display, future::Future};

    pub trait Tracing {
        /// Trace errors to the current trace span
        fn with_traced_errors(self) -> Self;
    }

    impl<A, E: Display> Tracing for Result<A, E> {
        fn with_traced_errors(self) -> Self {
            match self {
                Ok(x) => Ok(x),
                Err(e) => {
                    log_in_current_span(&e);
                    Err(e)
                }
            }
        }
    }

    #[async_trait]
    pub trait FutureTracing: Future {
        /// Trace errors to the current trace span
        async fn with_traced_errors(self) -> Self::Output;
    }

    #[async_trait]
    impl<A, E: Display, T: Future<Output = Result<A, E>> + Send> FutureTracing for T {
        async fn with_traced_errors(self: T) -> Result<A, E> {
            self.await.with_traced_errors()
        }
    }

    pub fn log_in_current_span<E: Display>(e: &E) {
        get_active_span(|span| {
            span.set_status(opentelemetry::trace::Status::Error {
                description: e.to_string().into(),
            });
        });
    }
}

mod test {
    #[test]
    fn test_append_path() {
        let url = reqwest::Url::parse("http://hasura.io").unwrap();
        let path = "capabilities";
        let result = crate::apis::default_api::append_path(&url, path).unwrap();
        assert_eq!(result.as_str(), "http://hasura.io/capabilities");
    }

    #[test]
    fn test_append_path_with_trailing_slash() {
        let url = reqwest::Url::parse("http://hasura.io/").unwrap();
        let path = "capabilities";
        let result = crate::apis::default_api::append_path(&url, path).unwrap();
        assert_eq!(result.as_str(), "http://hasura.io/capabilities");
    }

    #[test]
    fn test_append_path_with_non_empty_path() {
        let url = reqwest::Url::parse("http://hasura.io/ndc").unwrap();
        let path = "capabilities";
        let result = crate::apis::default_api::append_path(&url, path).unwrap();
        assert_eq!(result.as_str(), "http://hasura.io/ndc/capabilities");
    }

    #[test]
    fn test_append_path_with_non_empty_path_and_trailing_slash() {
        let url = reqwest::Url::parse("http://hasura.io/ndc/").unwrap();
        let path = "capabilities";
        let result = crate::apis::default_api::append_path(&url, path).unwrap();
        assert_eq!(result.as_str(), "http://hasura.io/ndc/capabilities");
    }
}
