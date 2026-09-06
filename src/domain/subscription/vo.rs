#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriptionProvider {
    GooglePlay,
    AppStore,
    Stripe,
    Paypay,
}

impl SubscriptionProvider {
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionProvider::GooglePlay => "google_play",
            SubscriptionProvider::AppStore => "app_store",
            SubscriptionProvider::Stripe => "stripte",
            SubscriptionProvider::Paypay => "paypay",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "google_play" => Some(SubscriptionProvider::GooglePlay),
            "app_store" => Some(SubscriptionProvider::AppStore),
            "stripe" => Some(SubscriptionProvider::Stripe),
            "paypay" => Some(SubscriptionProvider::Paypay),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriptionStatus {
    Active,
    Canceled,
    PastDue,
    Expired,
}

impl SubscriptionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            SubscriptionStatus::Active => "active",
            SubscriptionStatus::Canceled => "canceled",
            SubscriptionStatus::PastDue => "past_due",
            SubscriptionStatus::Expired => "expired",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "active" => Some(SubscriptionStatus::Active),
            "canceled" => Some(SubscriptionStatus::Canceled),
            "past_due" => Some(SubscriptionStatus::PastDue),
            "expired" => Some(SubscriptionStatus::Expired),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptionId(i64);

impl SubscriptionId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }

    pub fn value(&self) -> i64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderSubscriptionId(String);

impl ProviderSubscriptionId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
