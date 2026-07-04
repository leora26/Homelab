resource "zitadel_application_oidc" "tauri_client" {
  project_id = zitadel_project.pavuk.id
  name       = "Tauri Desktop Client"
  org_id     = var.zitadel_org_id

  app_type         = "OIDC_APP_TYPE_NATIVE"
  auth_method_type = "OIDC_AUTH_METHOD_TYPE_NONE"
  grant_types      = ["OIDC_GRANT_TYPE_AUTHORIZATION_CODE"]
  response_types   = ["OIDC_RESPONSE_TYPE_CODE"]

  # Issue self-contained JWT access tokens so the Rust backend can validate them
  # locally via JWKS (opaque bearer tokens cannot be decoded as JWTs).
  access_token_type = "OIDC_TOKEN_TYPE_JWT"

  redirect_uris = ["pavuk://callback"]
}

