use deadpool::{async_trait};
use deadpool::managed::{Manager, Metrics, Pool, RecycleResult};
use elasticsearch::Elasticsearch;
use elasticsearch::http::headers::{AUTHORIZATION, HeaderValue};
use elasticsearch::http::transport::{CloudConnectionPool, TransportBuilder};
use crate::errors::PhotonError;

// CloudConnectionPool only uses one connection. To create our own pool, we need to
// implement ConnectionPool from elasticsearch. We don't want to write our own
// connection pool code because that is a lot of effort and error-prone. To implement
// a trait for a type we don't own, we can wrap it in a struct (avoids orphan rules).
// However, the ConnectionPool trait implements next() which is expected to
// return Connection. The basic managed connection pool from the deadpool crate dishes
// out connections with get(), which returns an Object type that *behaves* like the
// target (Connection) but is not, itself, a Connection object. As such, we can't
// actually implement ConnectionPool as it will not compile without borrowing the
// connection from the Object, which will prevent it from being returned to the pool.

// tl;dr wrapping the entire client because the alternative is complicated

#[derive(Debug, Clone)]
pub struct ElasticConnectionManager {
    cloud_id: String,
    api_key: String
}

#[async_trait]
impl Manager for ElasticConnectionManager {
    type Type = Elasticsearch;
    type Error = PhotonError;

    async fn create(&self) -> Result<Elasticsearch, PhotonError> {
        let mut api_key_header = HeaderValue::from_str(&format!("ApiKey {}", &self.api_key))
            .map_err(|err| PhotonError::Internal(err.to_string()))?;

        api_key_header.set_sensitive(true);

        let fake_pool = CloudConnectionPool::new(&self.cloud_id)
            .map_err(PhotonError::Elasticsearch)?;

        let transport = TransportBuilder::new(fake_pool)
            .header(AUTHORIZATION, api_key_header)
            .build()
            .map_err(|err| PhotonError::Internal(err.to_string()))?;

        return Ok(Elasticsearch::new(transport));
    }

    async fn recycle(&self, _: &mut Elasticsearch, _: &Metrics) -> RecycleResult<PhotonError> {
        return Ok(());
    }
}

pub type ElasticConnectionPool = Pool<ElasticConnectionManager>;

pub fn create_connection_pool(cloud_id: String, api_key: String, size: usize) -> Result<ElasticConnectionPool, PhotonError> {
    let manager = ElasticConnectionManager { cloud_id, api_key };
    return Pool::builder(manager)
        .max_size(size)
        .build()
        .map_err(|err| PhotonError::Internal(err.to_string()));
}
