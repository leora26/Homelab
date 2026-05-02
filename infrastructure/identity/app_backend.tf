resource "zitadel_application_api" "rust_microservice" {
  project_id       = zitadel_project.pavuk.id
  org_id           = var.zitadel_org_id
  name             = "Rust Pavuk Backend API"
  auth_method_type = "API_AUTH_METHOD_TYPE_PRIVATE_KEY_JWT"
}

resource "zitadel_machine_user" "backend_svc_account" {
  org_id      = var.zitadel_org_id
  user_name   = "rust-backend-svc"
  name        = "Pavuk Backend Service Account"
  description = "Used by the user_management service to automate Zitadel"
}

resource "zitadel_machine_key" "backend_key" {
  org_id   = var.zitadel_org_id
  user_id  = zitadel_machine_user.backend_svc_account.id
  key_type = "KEY_TYPE_JSON"
}


output "backend_service_account_key" {
  value       = zitadel_machine_key.backend_key.key_details
  description = "Rust 'terraform output -raw backend_service_account_key > backend-key.json' after applying"
  sensitive   = true
}
