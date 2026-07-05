use async_trait::async_trait;
use tonic::{Request, Status};
use uuid::Uuid;
use crate::auth::identity_cache::CacheIdentityResolver;
use crate::auth::resolver::ExternalIdResolver;

#[async_trait]
pub trait RequestIdentityExt {
    async fn get_internal_id<R: ExternalIdResolver + Send + Sync> (
        &self,
        resolver: &CacheIdentityResolver<R>,
    ) -> Result<Uuid, Status>;
}

#[async_trait]
impl<T: Send + Sync> RequestIdentityExt for Request<T> {
    async fn get_internal_id<R: ExternalIdResolver + Send + Sync>(&self, resolver: &CacheIdentityResolver<R>) -> Result<Uuid, Status> {
        let zitadel_id = self.extensions().get::<String>().ok_or_else(|| {
            Status::internal("Critical: Bouncer interceptor failed to inject external ID")
        })?;

        resolve_internal_id(resolver, zitadel_id).await
    }
}

/// Resolves a Zitadel subject to the internal user UUID via the cached resolver.
///
/// Extracted from `RequestIdentityExt` so it can also be used on requests whose body
/// is not `Sync` (e.g. `Request<Streaming<..>>`), where the trait method is
/// unavailable: clone the `sub` out of the request extensions first, then call this.
pub async fn resolve_internal_id<R: ExternalIdResolver + Send + Sync>(
    resolver: &CacheIdentityResolver<R>,
    zitadel_id: &str,
) -> Result<Uuid, Status> {
    let internal_id_str = resolver.get_internal_id(zitadel_id).await.map_err(|e| {
        Status::permission_denied(format!("User profile not mapped in NAS: {}", e))
    })?;

    Uuid::parse_str(&internal_id_str).map_err(|e| {
        Status::internal(format!("Critical Data Error: Stored internal ID is not a valid UUID. {}", e))
    })
}