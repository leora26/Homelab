use tonic::service::Interceptor;
use crate::auth::auth::AuthState;

pub fn init_auth_interceptor(auth_state: AuthState) -> impl Interceptor + Clone {
    move |mut req: tonic::Request<()>| match req.metadata().get("authorization") {
        Some(token_header) => {
            let token_str = token_header.to_str().unwrap_or("").replace("Bearer ", "");

            let claims = auth_state.verify_token(&token_str)?;

            req.extensions_mut().insert(claims.sub);

            Ok(req)
        }
        None => Err(tonic::Status::unauthenticated(
            "Missing authorization header",
        )),
    }
}