use std::{env, error::Error, ffi::OsString, process};

use chrono::{DateTime, Utc};
use serde::Deserialize;

fn main() {
    let transactions = match parse() {
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
        Ok(value) => value,
    };

    for transaction in transactions.iter() {
        println!("{:#?}", transaction);
    }
}

#[derive(Debug, Deserialize)]
struct Transaction {
    description: String,
    date: DateTime<Utc>,
    category: String,
    amount: f64,
}

fn parse() -> Result<Vec<Transaction>, Box<dyn Error>> {
    let file_path = get_first_arg()?;
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

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
