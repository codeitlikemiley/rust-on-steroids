use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;

pub use cornucopia_async::Params;
pub use deadpool_postgres::{Pool, PoolError, Transaction};
use rustls::client::{ServerCertVerified, ServerCertVerifier};
use rustls::ServerName;
pub use tokio_postgres::Error as TokioPostgresError;

pub use queries::users::User;

pub fn create_pool(database_url: &str) -> deadpool_postgres::Pool {
    let config = tokio_postgres::Config::from_str(database_url).unwrap();

    let manager = if config.get_ssl_mode() != tokio_postgres::config::SslMode::Disable {
        let tls_config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(Arc::new(DummyTlsVerifier))
            .with_no_client_auth();

        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(tls_config);
        deadpool_postgres::Manager::new(config, tls)
    } else {
        deadpool_postgres::Manager::new(config, tokio_postgres::NoTls)
    };

    deadpool_postgres::Pool::builder(manager).build().unwrap()
}

struct DummyTlsVerifier;

impl ServerCertVerifier for DummyTlsVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: SystemTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }
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