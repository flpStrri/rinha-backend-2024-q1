use std::io::Error;
use tracing_subscriber::EnvFilter;

use rinha_backend_2023_q3::startup::Application;
use rinha_backend_2023_q3::{configuration, telemetry};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let subscriber = telemetry::get_subscriber(
        "rinha-de-backend-2023-q3",
        EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info")),
        std::io::stdout,
    );
    telemetry::init_subscriber(subscriber);

    let static_config = configuration::get_static_configuration().expect("failed to load configs");
    Application::build(static_config).await.run().await
}
