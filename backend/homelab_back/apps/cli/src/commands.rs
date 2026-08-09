use derive_new::new;

#[derive(new)]
pub struct ResizeCommand {
    pub requested_bytes: i64,
    pub force_shrink: bool,
}