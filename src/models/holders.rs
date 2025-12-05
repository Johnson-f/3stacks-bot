use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use finance_query_core::HolderType;
pub use finance_query_core::{
    InsiderPurchase, InsiderRosterMember, InsiderTransaction, InstitutionalHolder,
    MajorHoldersBreakdown, MutualFundHolder,
};

/// Aggregated holders data we expose to the bot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldersOverview {
    pub symbol: String,
    pub major_breakdown: Option<MajorHoldersBreakdown>,
    pub institutional_holders: Option<Vec<InstitutionalHolder>>,
    pub mutualfund_holders: Option<Vec<MutualFundHolder>>,
    pub insider_transactions: Option<Vec<InsiderTransaction>>,
    pub insider_purchases: Option<InsiderPurchase>,
    pub insider_roster: Option<Vec<InsiderRosterMember>>,
}

/// Convenience helpers for formatting dates.
pub fn parse_timestamp(value: &serde_json::Value) -> Option<DateTime<Utc>> {
    value
        .get("raw")
        .and_then(|r| r.as_i64())
        .and_then(|ts| DateTime::from_timestamp(ts, 0))
}

pub fn value_to_i64(value: &serde_json::Value) -> Option<i64> {
    value.get("raw").and_then(|r| r.as_i64()).or_else(|| value.as_i64())
}

pub fn value_to_f64(value: &serde_json::Value) -> Option<f64> {
    value.get("raw").and_then(|r| r.as_f64()).or_else(|| value.as_f64())
}

pub fn object_to_map(obj: &serde_json::Map<String, serde_json::Value>) -> HashMap<String, serde_json::Value> {
    obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}