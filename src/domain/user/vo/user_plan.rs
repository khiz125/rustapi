#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserPlan {
    Free,
    Premium,
}

impl UserPlan {
    pub fn as_str(&self) -> &str {
        match self {
            UserPlan::Free => "free",
            UserPlan::Premium => "premium",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "free" => Some(UserPlan::Free),
            "premium" => Some(UserPlan::Premium),
            _ => None,
        }
    }

    pub fn is_premium(&self) -> bool {
        matches!(self, UserPlan::Premium)
    }
}
