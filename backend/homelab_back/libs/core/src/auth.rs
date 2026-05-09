use std::sync::Arc;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tonic::Status;

#[derive(Debug, Deserialize, Serialize)]
pub struct ZitadelClaims {
    pub sub: String,
    pub aud: Vec<String>,
    pub exp: usize,
    pub iss: String
}

#[derive(Debug, Deserialize)]
struct JwksResponse {
    keys: Vec<serde_json::Value>
}

#[derive(Clone)]
pub struct AuthState {
    pub decoding_key: Arc<DecodingKey>,
    pub validation: Validation,
}

impl AuthState {
    pub async fn init(
        zitadel_domain: &str,
        target_client_id: &str
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let jwks_url = format!("{}/oauth/v2/keys", zitadel_domain);

        let jwks: JwksResponse = reqwest::get(&jwks_url).await?.json().await?;

        let key_data = &jwks.keys[0];
        let n = key_data.get("n")
            .and_then(|n| n.as_str())
            .ok_or("Missing 'n' in JWKS")?;
        let e = key_data.get("e")
            .and_then(|e| e.as_str())
            .ok_or("Missing 'e' in JWKS")?;

        let decoding_key = DecodingKey::from_rsa_components(n,e)?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[target_client_id]);
        validation.set_issuer(&[zitadel_domain]);
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);

        Ok(Self {
            decoding_key: Arc::new(decoding_key),
            validation
        })

    }

    pub fn verify_token (&self, token: &str) -> Result<ZitadelClaims, Status> {
        let token_data = decode::<ZitadelClaims>(
            token,
            &self.decoding_key,
            &self.validation,
        ).map_err(|e| Status::unauthenticated(format!("Invalid or expired token: {}", e)))?;

        Ok(token_data.claims)
    }
}