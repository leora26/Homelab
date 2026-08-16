use prost_types::Timestamp;

/// Protobuf timestamp to unix seconds.
///
/// The webview receives raw seconds and does its own formatting — it needs to sort
/// columns, render relative dates and filter by range, none of which work on a
/// pre-formatted string.
pub fn to_unix(ts: Option<Timestamp>) -> Option<i64> {
    ts.map(|ts| ts.seconds)
}
