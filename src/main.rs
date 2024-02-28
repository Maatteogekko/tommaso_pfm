/*
# Description

Banca Tommaso launched the Personal Finance Management (PFM) web-app you built, and it already gained 120k users, but unfortunately, only 15% of those are paying for it.
On average, every user spends €12.45 per month, which means the whole platform is generating a Monthly Recurring Revenue (MRR) of ~€225k (120000 * 12.45 * 0.15).
The bank had a target MRR of €1 million, so now they need to find a way to make the platform revenue go 4x.

After evaluating different proposals, your manager decided to proceed with your solution (little does she know it was actually ChatGPT that came up with it).
The idea is to sell the banking services that run in the backend of the platform to other banks.
For this reason, you are working on the library that the other banks will use to interact with those services.

# Tasks

Implement a function that, given the location of a file containing a list of transactions, returns a list of objects with the following type:
```
{
    description: String,
    amount: Number,
    date: Date,
    category: String,
}
```
Here is an example file of transactions: transactions.txt
For now, we will call this structure a Transaction.

- Implement a function that, given a list of transactions, returns the information of how much was spent on each category found in the transaction list.
- Implement a function that, given a list of transactions, returns the information of how much was spent in each single month.
- Implement a function that, given a list of transactions, returns the information of how much was spent per month on average.
- BONUS: Implement a function that, given a list of transactions, returns how much was spent each month per each category.
*/

use std::{collections::HashMap, env, error::Error, ffi::OsString, fmt, process};

use chrono::{DateTime, Datelike, Utc};
use serde::Deserialize;

fn main() {
    // wether incomes should be considered "negative spendings".
    let relative = true;

    let mut transactions = match get_first_arg().and_then(parse) {
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
        Ok(value) => value,
    };

    println!("-> Transactions:");
    for transaction in transactions.iter() {
        println!("• {}", transaction);
    }
    println!();

    if !relative {
        transactions.retain(|v| v.amount < 0 as f64);
    }

    println!("-> Total spending per category:");
    for (key, value) in spending_per_category(&transactions) {
        println!("{}: {:.2}", key, value);
    }
    println!();

    println!("-> Total spending per month:");
    for (key, value) in spending_per_month(&transactions) {
        println!("{}: {:.2}", key, value);
    }
    println!();

    println!(
        "-> Average month spending: {:.2}\n",
        spending_month_average(&transactions)
    );

    println!("-> Total spending per month per category:");
    for (key, value) in spending_per_month_per_category(&transactions) {
        println!("{}:", key);
        for (key, value) in value {
            println!("\t{}: {:.2}", key, value);
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct Transaction {
    description: String,
    date: DateTime<Utc>,
    category: String,
    amount: f64,
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

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn parse(file_path: OsString) -> Result<Vec<Transaction>, Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path(file_path)?;

    let mut transactions = Vec::new();
    for result in reader.deserialize() {
        let record: Transaction = result?;
        transactions.push(record);
    }
    Ok(transactions)
}

fn spending_per_category(transactions: &[Transaction]) -> HashMap<String, f64> {
    let mut map: HashMap<String, f64> = HashMap::new();
    for transaction in transactions {
        *map.entry(transaction.category.clone()).or_default() += transaction.amount;
    }
    map
}

fn spending_per_month(transactions: &[Transaction]) -> HashMap<String, f64> {
    let mut map: HashMap<String, f64> = HashMap::new();
    for transaction in transactions {
        *map.entry(transaction.month()).or_default() += transaction.amount;
    }
    map
}

fn spending_month_average(transactions: &[Transaction]) -> f64 {
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
fn spending_per_month_per_category(
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
