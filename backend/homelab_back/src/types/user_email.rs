pub struct UserEmail(String);

impl UserEmail {
    pub fn parse(s: String) -> Result<Self, String> {
        if s.trim().is_empty() {
            return Err("Email cannot be empty".to_string());
        } else if !s.contains("@") {
            return Err("Email is missing '@' symbol".to_string());
        }

        Ok(Self(s))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}