use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Matches finance-query-core statement types for fundamentals timeseries.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatementType {
    #[serde(rename = "income")]
    IncomeStatement,
    #[serde(rename = "balance")]
    BalanceSheet,
    #[serde(rename = "cashflow")]
    CashFlow,
}

impl StatementType {
    pub fn as_str(&self) -> &'static str {
        match self {
            StatementType::IncomeStatement => "income",
            StatementType::BalanceSheet => "balance",
            StatementType::CashFlow => "cashflow",
        }
    }
}

/// Matches finance-query-core frequencies for statements.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Frequency {
    #[serde(rename = "annual")]
    Annual,
    #[serde(rename = "quarterly")]
    Quarterly,
}

impl Frequency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Frequency::Annual => "annual",
            Frequency::Quarterly => "quarterly",
        }
    }
}

/// Mirror of finance-query-core FinancialStatement for raw timeseries data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct FinancialStatement {
    pub symbol: String,
    pub statement_type: String,
    pub frequency: String,
    #[serde(rename = "statement")]
    pub statement: HashMap<String, HashMap<String, serde_json::Value>>,
}

/// Bot-facing financial snapshot for a ticker (used in Discord responses).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialSummary {
    pub symbol: String,
    pub revenue: Option<f64>,
    pub eps: Option<f64>,
    pub pe_ratio: Option<f64>,
    pub market_cap: Option<f64>,
    pub currency: Option<String>,
}