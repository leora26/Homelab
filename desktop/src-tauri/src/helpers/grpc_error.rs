use tonic::{Code, Status};

/// Turns a gRPC failure into a message meant for a person.
///
/// The backend already puts user-facing text in the status message — "No free space is
/// available", "A file with the same name already exists in the given folder" — so the
/// job here is to pass that through instead of burying it in transport detail. Only the
/// codes whose messages are internal (or empty) get substituted.
///
/// The full status is still logged, so the technical detail isn't lost.
pub fn grpc_message(status: &Status) -> String {
    eprintln!(
        "🛑 gRPC [{:?}] {}",
        status.code(),
        status.message()
    );

    let message = status.message().trim();

    match status.code() {
        Code::Unavailable => "Can't reach the Pavuk server. Check that it's running.".to_string(),
        Code::Unauthenticated => "Your session has expired. Please sign in again.".to_string(),
        Code::PermissionDenied => "You don't have permission to do that.".to_string(),
        Code::DeadlineExceeded => "The server took too long to respond.".to_string(),

        // "A database error occurred" and "Internal server error" are what the backend
        // sends when it deliberately withholds detail; neither helps the reader.
        Code::Internal
            if message.is_empty()
                || message == "Internal server error"
                || message == "A database error occurred" =>
        {
            "Something went wrong on the server.".to_string()
        }

        Code::NotFound if message.is_empty() => "That item no longer exists.".to_string(),

        _ if !message.is_empty() => message.to_string(),
        _ => "The request failed.".to_string(),
    }
}
