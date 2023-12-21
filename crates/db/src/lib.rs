use std::str::FromStr;
use std::sync::Arc;
pub use cornucopia_async::Params;
pub use deadpool_postgres::{Pool, PoolError, Transaction};
pub use tokio_postgres::Error as TokioPostgresError;

pub use queries::users::User;


#[cfg(feature = "production")]
fn create_tls_config() -> rustls::ClientConfig {
    let mut root_cert_store = rustls::RootCertStore::empty();

    // Load the certificate file.
    // or on local machine:
    // cargo run --bin ssl_generator
    let cert_file = include_str!("../../../cert/cert.pem"); // replace with your cert.pem path
    let mut reader = std::io::Cursor::new(cert_file);
    let certs = rustls_pemfile::certs(&mut reader)
        .map(|cert_result| cert_result
            .map(|der_cert| rustls::Certificate(der_cert.to_vec()))
            .expect("Failed to read certificate"))
        .collect::<Vec<_>>();

    root_cert_store.add_parsable_certificates(&certs);

    let config = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    config
}

#[cfg(not(feature = "production"))]
fn create_tls_config() -> rustls::ClientConfig {
    let  root_cert_store = rustls::RootCertStore::empty();
    // Production environment: Secure TLS configuration.
    // Use rustls' built-in verifier or implement a custom verifier.


    // Add certificate loading and verification logic here.
    rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(Arc::new(root_cert_store))
        .with_no_client_auth()
}


pub fn create_pool(database_url: &str) -> deadpool_postgres::Pool {

    let config = tokio_postgres::Config::from_str(database_url).unwrap();

    let manager = if config.get_ssl_mode() != tokio_postgres::config::SslMode::Disable {
        let tls_config = create_tls_config();

        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(tls_config);
        deadpool_postgres::Manager::new(config, tls)
    } else {
        deadpool_postgres::Manager::new(config, tokio_postgres::NoTls)
    };

    deadpool_postgres::Pool::builder(manager).build().unwrap()
}

include!(concat!(env!("OUT_DIR"), "/cornucopia.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    fn init() {
        dotenv::dotenv().ok();
    }

    #[tokio::test]
    async fn load_users() {
        init();
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = create_pool(&db_url);

        let client = pool.get().await.unwrap();

        let users = crate::queries::users::get_users()
            .bind(&client)
            .all()
            .await
            .unwrap();

        dbg!(users);
    }
}