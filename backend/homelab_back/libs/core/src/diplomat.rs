use std::sync::{Arc};
use std::time::{SystemTime, UNIX_EPOCH};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Deserialize, Clone)]
pub struct ZitadelMachineKey {
    #[serde(rename = "type")]
    pub key_type: String,
    #[serde(rename = "keyId")]
    pub key_id: String,
    pub key: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "expirationDate")]
    pub expiration_date: Option<String>,
}

#[derive(Serialize)]
struct AssertionClaims {
    iss: String,
    sub: String,
    aud: String,
    iat: usize,
    exp: usize,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

pub struct CachedToken {
    access_token: String,
    expires_at: u64,
}

pub struct ZitadelDiplomat {
    domain: String,
    machine_key: ZitadelMachineKey,
    http_client: reqwest::Client,
    cached_token: Arc<RwLock<Option<CachedToken>>>,
}

impl ZitadelDiplomat {
    // Bootstraps the Diplomat by reading your backend-key.json from disk
    pub async fn new (domain: String, key_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let key_content = tokio::fs::read_to_string(key_path).await?;
        let machine_key: ZitadelMachineKey = serde_json::from_str(&key_content)?;

        Ok(Self {
            domain,
            machine_key,
            http_client: reqwest::Client::new(),
            cached_token: Arc::new(RwLock::new(None)),
        })
    }

    pub async fn get_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        {
            let cache = self.cached_token.read().await;
            if let Some(token) = cache.as_ref() {
                let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

                if now < token.expires_at - 10 {
                    return Ok(token.access_token.clone());
                }
            }
        }

        let mut cache = self.cached_token.write().await;

        if let Some(token) = cache.as_ref() {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            if now < token.expires_at - 10 {
                return Ok(token.access_token.clone())
            }
        }

        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
        let claims = AssertionClaims {
            iss: self.machine_key.user_id.clone(),
            sub: self.machine_key.user_id.clone(),
            aud: self.domain.clone(),
            iat: now,
            exp: now + 3600,
        };

        let mut header = Header::new(Algorithm::RS256);
        header.kid = Some(self.machine_key.key_id.clone());

        let encoding_key = EncodingKey::from_rsa_pem(self.machine_key.key.as_bytes())?;
        let assertion = encode(&header, &claims, &encoding_key)?;

        let token_url = format!("{}/oauth/v2/token", self.domain);
        let params = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &assertion),
            // This scope grants the bot permission to call the Zitadel Management APIs
            ("scope", "openid profile urn:zitadel:iam:org:project:id:zitadel:aud"),
        ];

        let res = self.http_client.post(&token_url).form(&params).send().await?;

        if !res.status().is_success() {
            let err_text = res.text().await?;
            return Err(format!("Zitadel rejected assertion {}", err_text).into());
        }

        let token_res: TokenResponse = res.json().await?;

        let expires_at = (now as  u64) + token_res.expires_in;
        let access_token = token_res.access_token;

        *cache = Some(CachedToken {
            access_token: access_token.clone(),
            expires_at,
        });

        Ok(access_token)
    }
}