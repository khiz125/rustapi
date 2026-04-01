#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OAuthProvider {
    Google,
    Apple,
}

impl OAuthProvider {
    pub fn as_str(&self) -> &str {
        match self {
            OAuthProvider::Google => "google",
            OAuthProvider::Apple => "apple",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "google" => Some(OAuthProvider::Google),
            "apple" => Some(OAuthProvider::Apple),
            _ => None,
        }
    }
}
