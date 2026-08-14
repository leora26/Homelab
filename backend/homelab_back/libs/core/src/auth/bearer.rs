use crate::auth::auth::AuthState;
use crate::auth::identity_cache::CacheIdentityResolver;
use crate::auth::resolver::ExternalIdResolver;
use uuid::Uuid;

/// Failure resolving a caller's identity from a bearer token.
///
/// Transport-agnostic on purpose: callers (actix REST, tonic gRPC, …) map these
/// variants onto their own error/status types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthError {
    /// No `Authorization` header, or the token was empty.
    MissingToken,
    /// Token failed signature / issuer / audience / expiry validation.
    InvalidToken,
    /// Token is valid but its subject is not mapped to a local user.
    NotProvisioned,
    /// The stored internal id is not a valid UUID (data corruption).
    MalformedInternalId,
    /// Token is valid and mapped, but the account has been blocked by an admin.
    Blocked,
}

impl AuthError {
    pub fn message(&self) -> &'static str {
        match self {
            AuthError::MissingToken => "Missing authorization header",
            AuthError::InvalidToken => "Invalid or expired token",
            AuthError::NotProvisioned => "User profile not mapped",
            AuthError::MalformedInternalId => "Stored internal id is not a valid UUID",
            AuthError::Blocked => "Account is blocked",
        }
    }
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message())
    }
}

impl std::error::Error for AuthError {}

/// Validates a raw `Authorization` header value and resolves the caller's internal
/// user id (`sub` → cached `external_id` lookup).
///
/// Shared across services that authenticate outside the gRPC interceptor (e.g. the
/// actix REST endpoints). Pass the header value verbatim; the `Bearer ` prefix is
/// stripped here.
pub async fn resolve_caller_id<R: ExternalIdResolver + Send + Sync>(
    auth_state: &AuthState,
    resolver: &CacheIdentityResolver<R>,
    authorization_header: Option<&str>,
) -> Result<Uuid, AuthError> {
    let header = authorization_header.ok_or(AuthError::MissingToken)?;
    let token = header.strip_prefix("Bearer ").unwrap_or(header).trim();
    if token.is_empty() {
        return Err(AuthError::MissingToken);
    }

    let claims = auth_state
        .verify_token(token)
        .map_err(|_| AuthError::InvalidToken)?;

    let internal_id = resolver
        .get_internal_id(&claims.sub)
        .await
        .map_err(|_| AuthError::NotProvisioned)?;

    let internal_id = Uuid::parse_str(&internal_id).map_err(|_| AuthError::MalformedInternalId)?;

    // Block guard (mirrors resolve_internal_id for the gRPC path). Live check; a lookup
    // failure fails closed by denying the request.
    if resolver
        .is_blocked(internal_id)
        .await
        .map_err(|_| AuthError::NotProvisioned)?
    {
        return Err(AuthError::Blocked);
    }

    Ok(internal_id)
}
