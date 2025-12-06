use chrono::{DateTime, Utc};
use finance_query_core::{HolderType, YahooError, YahooFinanceClient};
use serde_json::Value;

use crate::models::holders::{object_to_map, parse_timestamp, value_to_f64, value_to_i64};
use crate::models::{
    HoldersOverview, InsiderPurchase, InsiderRosterMember, InsiderTransaction, InstitutionalHolder,
    MajorHoldersBreakdown, MutualFundHolder,
};

pub async fn fetch_holders(
    client: &YahooFinanceClient,
    symbol: &str,
    holder_type: HolderType,
) -> Result<HoldersOverview, YahooError> {
    let modules = match holder_type {
        HolderType::Major => vec!["majorHoldersBreakdown"],
        HolderType::Institutional => vec!["institutionOwnership"],
        HolderType::MutualFund => vec!["fundOwnership"],
        HolderType::InsiderTransactions | HolderType::InsiderPurchases => {
            vec!["insiderTransactions"]
        }
        HolderType::InsiderRoster => vec!["insiderHolders"],
    };

    let data = client.get_quote_summary(symbol, &modules).await?;

    let result = data
        .get("quoteSummary")
        .and_then(|q| q.get("result"))
        .and_then(|r| r.get(0))
        .ok_or_else(|| YahooError::ParseError("missing quoteSummary.result".to_string()))?;

    let mut overview = HoldersOverview {
        symbol: symbol.to_uppercase(),
        major_breakdown: None,
        institutional_holders: None,
        mutualfund_holders: None,
        insider_transactions: None,
        insider_purchases: None,
        insider_roster: None,
    };

    match holder_type {
        HolderType::Major => {
            overview.major_breakdown = parse_major_breakdown(result);
        }
        HolderType::Institutional => {
            overview.institutional_holders = parse_institutional_holders(result);
        }
        HolderType::MutualFund => {
            overview.mutualfund_holders = parse_mutualfund_holders(result);
        }
        HolderType::InsiderTransactions | HolderType::InsiderPurchases => {
            let txs = parse_insider_transactions(result);
            if holder_type == HolderType::InsiderTransactions {
                overview.insider_transactions = txs.clone();
            }
            if holder_type == HolderType::InsiderPurchases {
                overview.insider_purchases = parse_insider_purchases(txs);
            }
        }
        HolderType::InsiderRoster => {
            overview.insider_roster = parse_insider_roster(result);
        }
    }

    Ok(overview)
}

fn parse_major_breakdown(result: &Value) -> Option<MajorHoldersBreakdown> {
    let obj = result.get("majorHoldersBreakdown")?.as_object()?;
    Some(MajorHoldersBreakdown {
        breakdown_data: object_to_map(obj),
    })
}

fn parse_institutional_holders(result: &Value) -> Option<Vec<InstitutionalHolder>> {
    let list = result
        .get("institutionOwnership")
        .and_then(|v| v.get("ownershipList"))
        .and_then(|v| v.as_array())?;

    let mut holders = Vec::new();
    for item in list {
        let holder = item
            .get("organization")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();
        if holder.is_empty() {
            continue;
        }

        let shares = item
            .get("position")
            .and_then(value_to_i64)
            .unwrap_or_default();
        let date_reported = item
            .get("reportDate")
            .and_then(parse_timestamp)
            .unwrap_or_else(|| DateTime::<Utc>::from_timestamp(0, 0).unwrap());

        let percent_out = item
            .get("pctHeld")
            .and_then(value_to_f64)
            .or_else(|| item.get("percentOut").and_then(value_to_f64));
        let value = item.get("value").and_then(value_to_i64);

        holders.push(InstitutionalHolder {
            holder,
            shares,
            date_reported,
            percent_out,
            value,
        });
    }

    if holders.is_empty() {
        None
    } else {
        Some(holders)
    }
}

fn parse_mutualfund_holders(result: &Value) -> Option<Vec<MutualFundHolder>> {
    let list = result
        .get("fundOwnership")
        .and_then(|v| v.get("ownershipList"))
        .and_then(|v| v.as_array())?;

    let mut holders = Vec::new();
    for item in list {
        let holder = item
            .get("organization")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();
        if holder.is_empty() {
            continue;
        }

        let shares = item
            .get("position")
            .and_then(value_to_i64)
            .unwrap_or_default();
        let date_reported = item
            .get("reportDate")
            .and_then(parse_timestamp)
            .unwrap_or_else(|| DateTime::<Utc>::from_timestamp(0, 0).unwrap());

        let percent_out = item
            .get("pctHeld")
            .and_then(value_to_f64)
            .or_else(|| item.get("percentOut").and_then(value_to_f64));
        let value = item.get("value").and_then(value_to_i64);

        holders.push(MutualFundHolder {
            holder,
            shares,
            date_reported,
            percent_out,
            value,
        });
    }

    if holders.is_empty() {
        None
    } else {
        Some(holders)
    }
}

fn parse_insider_transactions(result: &Value) -> Option<Vec<InsiderTransaction>> {
    let list = result
        .get("insiderTransactions")
        .and_then(|v| v.get("transactions"))
        .and_then(|v| v.as_array())?;

    let mut out = Vec::new();
    for item in list {
        let insider = item
            .get("filerName")
            .or_else(|| item.get("insider"))
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();
        if insider.is_empty() {
            continue;
        }
        let position = item
            .get("filerRelation")
            .or_else(|| item.get("position"))
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();
        let transaction = item
            .get("transactionText")
            .or_else(|| item.get("transaction"))
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();
        let start_date = item
            .get("startDate")
            .and_then(parse_timestamp)
            .unwrap_or_else(|| DateTime::<Utc>::from_timestamp(0, 0).unwrap());
        let shares = item.get("shares").and_then(value_to_i64);
        let value = item.get("value").and_then(value_to_i64);
        let ownership = item
            .get("ownership")
            .or_else(|| item.get("filerRelation"))
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());

        out.push(InsiderTransaction {
            start_date,
            insider,
            position,
            transaction,
            shares,
            value,
            ownership,
        });
    }

    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn parse_insider_purchases(txs: Option<Vec<InsiderTransaction>>) -> Option<InsiderPurchase> {
    let txs = txs?;
    if txs.is_empty() {
        return None;
    }

    let mut purchases_shares: i64 = 0;
    let mut purchases_tx = 0;
    let mut sales_shares: i64 = 0;
    let mut sales_tx = 0;

    for tx in &txs {
        let text = tx.transaction.to_lowercase();
        let shares = tx.shares.unwrap_or(0);
        if text.contains("buy") || text.contains("purchase") {
            purchases_shares += shares;
            purchases_tx += 1;
        } else if text.contains("sell") || text.contains("sale") {
            sales_shares += shares;
            sales_tx += 1;
        }
    }

    Some(InsiderPurchase {
        period: "recent".to_string(),
        purchases_shares: Some(purchases_shares),
        purchases_transactions: Some(purchases_tx),
        sales_shares: Some(sales_shares),
        sales_transactions: Some(sales_tx),
        net_shares: Some(purchases_shares - sales_shares),
        net_transactions: Some(purchases_tx - sales_tx),
        total_insider_shares: None,
        net_percent_insider_shares: None,
        buy_percent_insider_shares: None,
        sell_percent_insider_shares: None,
    })
}

fn parse_insider_roster(result: &Value) -> Option<Vec<InsiderRosterMember>> {
    let list = result
        .get("insiderHolders")
        .and_then(|v| v.get("holders"))
        .and_then(|v| v.as_array())?;

    let mut out = Vec::new();
    for item in list {
        let name = item
            .get("name")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();
        if name.is_empty() {
            continue;
        }
        let position = item
            .get("relation")
            .or_else(|| item.get("position"))
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string();
        let most_recent_transaction = item
            .get("transactionDescription")
            .or_else(|| item.get("mostRecentTransaction"))
            .and_then(|s| s.as_str())
            .map(|s| s.to_string());
        let latest_transaction_date = item
            .get("latestTransDate")
            .or_else(|| item.get("latestTransactionDate"))
            .and_then(parse_timestamp);
        let shares_owned_directly = item
            .get("positionDirect")
            .or_else(|| item.get("sharesOwnedDirectly"))
            .and_then(value_to_i64);
        let shares_owned_indirectly = item
            .get("positionIndirect")
            .or_else(|| item.get("sharesOwnedIndirectly"))
            .and_then(value_to_i64);
        let position_direct_date = item.get("positionDirectDate").and_then(parse_timestamp);

        out.push(InsiderRosterMember {
            name,
            position,
            most_recent_transaction,
            latest_transaction_date,
            shares_owned_directly,
            shares_owned_indirectly,
            position_direct_date,
        });
    }

    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}
