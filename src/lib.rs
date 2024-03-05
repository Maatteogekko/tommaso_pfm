use chrono::{DateTime, Datelike, Utc};
use serde::Deserialize;
use std::{collections::HashMap, error::Error, ffi::OsString, fmt};

#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    pub description: String,
    pub date: DateTime<Utc>,
    pub category: String,
    pub amount: f64,
}

impl Transaction {
    fn month(&self) -> String {
        format!("{}-{}", self.date.year(), self.date.month())
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\n\tDate: {}\n\tCategory: {}\n\tAmount: {:.2}",
            self.description,
            self.date.format("%Y-%m-%d %H:%M:%S"),
            self.category,
            self.amount
        )
    }
}

/// Parses the `file_path` for valid Transactions.
/// A count of malformed records is also returned.
pub fn parse(file_path: OsString) -> Result<(Vec<Transaction>, u8), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .flexible(true)
        .from_path(file_path)?;

    let mut malformed = 0;
    let mut transactions = Vec::new();
    for result in reader.deserialize() {
        match result {
            Ok(transaction) => transactions.push(transaction),
            Err(_) => malformed += 1,
        };
    }
    Ok((transactions, malformed))
}

pub fn spending_per_category(transactions: &[Transaction]) -> HashMap<String, f64> {
    let mut map: HashMap<String, f64> = HashMap::new();
    for transaction in transactions {
        *map.entry(transaction.category.clone()).or_default() += transaction.amount;
    }
    map
}

pub fn spending_per_month(transactions: &[Transaction]) -> HashMap<String, f64> {
    let mut map: HashMap<String, f64> = HashMap::new();
    for transaction in transactions {
        *map.entry(transaction.month()).or_default() += transaction.amount;
    }
    map
}

pub fn spending_month_average(transactions: &[Transaction]) -> f64 {
    let month_spendings = spending_per_month(transactions);
    let sum: f64 = month_spendings.values().sum();
    let count = month_spendings.len();
    if count > 0 {
        sum / count as f64
    } else {
        0.0
    }
}

/// Returns a map of `<Month, <Category, amount>>`.
pub fn spending_per_month_per_category(
    transactions: &[Transaction],
) -> HashMap<String, HashMap<String, f64>> {
    // first cluster the transactions by month
    let mut months_map: HashMap<String, Vec<Transaction>> = HashMap::new();
    for transaction in transactions {
        months_map
            .entry(transaction.month())
            .or_default()
            .push(transaction.clone());
    }

    // then process spending for each cluster
    months_map
        .into_iter()
        .map(|(month, transactions)| (month, spending_per_category(&transactions)))
        .collect()
}
