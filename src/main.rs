use std::{env, error::Error, ffi::OsString, process};

use tommaso_pfm::{
    parse, spending_month_average, spending_per_category, spending_per_month,
    spending_per_month_per_category,
};

fn main() {
    // wether incomes should be considered "negative spendings".
    let relative = true;

    let (mut transactions, skipped) = match get_first_arg().and_then(parse) {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };
    if skipped > 0 {
        println!("WARNING Skipped {} malformed transactions", skipped);
    }

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

/// Returns the first positional argument sent to this process. If there are no
/// positional arguments, then this returns an error.
fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        Some(file_path) => Ok(file_path),
        None => Err(From::from("expected 1 argument, but got none")),
    }
}
