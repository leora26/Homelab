variable zitadel_token {
  type        = string
  sensitive   = true
  description = "Personal Access Token for set up Service account"
}


variable "zitadel_org_id" {
  type        = string
  description = "The absolute ID of the default Zitadel Organization"
}