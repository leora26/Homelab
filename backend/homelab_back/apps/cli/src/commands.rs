use derive_new::new;

#[derive(new)]
pub struct ResizeCommand {
    pub requested_bytes: i64,
    pub force_shrink: bool,
}

#[derive(new)]
pub struct ListFileCommand {
    pub limit: i64,
    pub file_type: Option<i32>,
}

#[derive(new)]
pub struct FindFileCommand {
    pub prefix: String
}

#[derive(new)]
pub struct GetVersionsCommand {
    pub prefix: String,
    pub limit: i64,
}

#[derive(new)]
pub struct ListUserCommand {
    pub limit: i64,
    pub is_blocked: Option<bool>
}

#[derive(new)]
pub struct FindUserCommand {
    pub query: String
}

#[derive(new)]
pub struct GetUserVersionCommand {
    pub query: String,
    pub limit: i64
}

#[derive(new)]
pub struct ToggleBlockCommand {
    pub user_id: String,
    pub is_blocked: bool
}

#[derive(new)]
pub struct SetQuotaCommand {
    pub user_id: String,
    pub allowed_storage: i64
}