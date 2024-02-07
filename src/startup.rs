use std::io::Error;
use std::net::{Ipv4Addr, SocketAddr};

use axum::routing::{get, post};
use axum::{http, Router};
use mongodb::options::ClientOptions;
use mongodb::{Client, Database};
use tower::ServiceBuilder;
use tower_http::request_id::{MakeRequestId, RequestId};
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};
use uuid::Uuid;

use crate::configuration::{DatabaseConfiguration, StaticConfiguration};
use crate::routes;

pub struct Application {
    app: Router,
    listener: tokio::net::TcpListener,
}

impl Application {
    pub async fn build(static_config: StaticConfiguration) -> Self {
        let server_address =
            SocketAddr::from((Ipv4Addr::UNSPECIFIED, static_config.application_port));
        let server_listener = tokio::net::TcpListener::bind(server_address)
            .await
            .expect("failed to bind random port");

        let mongodb_pool = get_database_connection(static_config.database)
            .await
            .expect("failed to connect to mongodb");

        let sensitive_headers: std::sync::Arc<[_]> =
            vec![http::header::AUTHORIZATION, http::header::COOKIE].into();
        let tracing_middleware = ServiceBuilder::new()
            .sensitive_request_headers(sensitive_headers.clone())
            .set_x_request_id(MakeRequestUuid)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(
                        DefaultMakeSpan::new()
                            .include_headers(true)
                            .level(tracing::Level::INFO),
                    )
                    .on_response(DefaultOnResponse::new().include_headers(true)),
            )
            .propagate_x_request_id()
            .sensitive_response_headers(sensitive_headers);

        let app = Router::new()
            .route("/pessoas/:id", get(routes::devs::get_person))
            .route("/pessoas", post(routes::devs::create_person))
            .route("/pessoas", get(routes::devs::search_persons))
            .route("/contagem-pessoas", get(routes::count_devs::count_persons))
            .layer(tracing_middleware)
            .route("/health-check", get(routes::health_check::health_check))
            .with_state(mongodb_pool);

        Application {
            app,
            listener: server_listener,
        }
    }

    pub async fn run(self) -> Result<(), Error> {
        axum::serve(self.listener, self.app).await
    }

    pub fn address(&self) -> String {
        format!("{}", self.listener.local_addr().unwrap())
    }
}

pub async fn get_database_connection(
    database_config: DatabaseConfiguration,
) -> Result<Database, mongodb::error::Error> {
    let client_options = ClientOptions::parse(database_config.connection_string()).await?;
    let client = Client::with_options(client_options)?;
    Ok(client.database(&database_config.database_name))
}

#[derive(Clone, Copy)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _request: &http::Request<B>) -> Option<RequestId> {
        let request_id = Uuid::new_v4().to_string().parse().unwrap();
        Some(RequestId::new(request_id))
    }
}
