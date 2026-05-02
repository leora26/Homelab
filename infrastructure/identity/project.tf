resource "zitadel_project" "pavuk" {
  name                   = "Pavuk NAS"
  project_role_assertion = true
  project_role_check     = true
  has_project_check      = true
  org_id = var.zitadel_org_id
}
