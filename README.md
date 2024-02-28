[Programming Mentoring challenge](https://discord.com/channels/1130043756477960256/1130173129684160634/1211374987743596595)

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