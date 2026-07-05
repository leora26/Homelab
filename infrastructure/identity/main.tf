terraform {
  required_providers {
    zitadel = {
      source  = "zitadel/zitadel"
      version = "2.12.6"
    }
  }
}

provider "zitadel" {
  domain       = "localhost"
  insecure     = "true"
  port         = "8085"
  access_token = var.zitadel_token
}
