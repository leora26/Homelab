use tonic::{Request, Status};

pub fn with_auth(token: String) -> impl Fn(Request<()>) -> Result<Request<()>, Status> {
    move |mut req: Request<()>| {
        let auth_header = format!("Bearer {}", token);

        // --- DEBUG TRACE ---
        // A standard JWT will always start with "eyJ".
        // If this prints a random string of characters, Zitadel gave you an opaque token.
        println!("🚀 Injecting Token: {}...", &auth_header.chars().take(25).collect::<String>());

        // Tonic requires metadata keys to be strictly lowercase
        match tonic::metadata::MetadataValue::try_from(&auth_header) {
            Ok(meta_value) => {
                req.metadata_mut().insert("authorization", meta_value);
            }
            Err(e) => {
                eprintln!("❌ Failed to parse authorization header: {}", e);
            }
        }
        Ok(req)
    }
}