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

        let internal_id_str = resolver.get_internal_id(zitadel_id).await.map_err(|e| {
            Status::permission_denied(format!("User profile not mapped in NAS: {}", e))
        })?;

        Uuid::parse_str(&internal_id_str).map_err(|e| {
            Status::internal(format!("Critical Data Error: Stored internal ID is not a valid UUID. {}", e))
        })
    }
}