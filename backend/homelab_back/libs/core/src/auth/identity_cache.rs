use std::time::Duration;
use moka::future::Cache;
use crate::auth::resolver::ExternalIdResolver;

pub struct CacheIdentityResolver<R: ExternalIdResolver> {
    cache: Cache<String, String>,
    resolver: R
}

impl<R: ExternalIdResolver> CacheIdentityResolver<R> {
    pub fn new(resolver: R) -> Self {
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(900))
            .max_capacity(10_000)
            .build();

        Self {cache, resolver}
    }

    pub async fn get_internal_id (&self, external_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(internal_id) = self.cache.get(external_id).await {
            return Ok(internal_id)
        }

        let internal_id = self.resolver.resolve_external_id(external_id).await?;

        self.cache.insert(external_id.to_string(), internal_id.clone()).await;

        Ok(internal_id)
    }

    pub async fn invalidate_cache (&self, external_id: &str) {
        self.cache.remove(external_id).await;
    }
}