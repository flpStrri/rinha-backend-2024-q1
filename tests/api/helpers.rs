use std::sync::Once;

use mongodb::Database;
use tracing_subscriber::EnvFilter;

use rinha_backend_2023_q3::startup::{get_database_connection, Application};
use rinha_backend_2023_q3::{configuration, telemetry};

static TRACING: Once = Once::new();

pub struct TestApp {
    pub address: String,
    pub mongodb_pool: Database,
}

pub async fn spawn_app() -> TestApp {
    TRACING.call_once(|| {
        let default_filter_level = EnvFilter::new("info");
        let subscriber_name = "rinha-de-backend-2023-q3";

        if std::env::var("TEST_LOG").is_ok() {
            let subscriber =
                telemetry::get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
            telemetry::init_subscriber(subscriber);
        } else {
            let subscriber =
                telemetry::get_subscriber(subscriber_name, default_filter_level, std::io::sink);
            telemetry::init_subscriber(subscriber);
        };
    });

    let mut static_config =
        configuration::get_static_configuration().expect("failed to load configs");
    let test_database_name = format!("test-{}", &ulid::Ulid::new().to_string());
    println!("Test using database name: {}", test_database_name);
    static_config.database.database_name = test_database_name;

    let database_test_pool = get_database_connection(static_config.database.clone())
        .await
        .expect("failed to connect to mongodb");
    let application = Application::build(static_config).await;
    let address = format!("http://{}", application.address());

    tokio::spawn(async move { application.run().await.expect("Failed to run the server") });
    TestApp {
        address,
        mongodb_pool: database_test_pool,
    }
}
