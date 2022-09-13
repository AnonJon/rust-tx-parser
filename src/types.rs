use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Transactions {
    pub transactions: Vec<Transaction>,
    // pub removed: Vec<String>,
    // pub lastblock: String,
}

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub account: String,
    pub address: String,
    pub category: String,
    pub amount: f64,
    pub vout: i64,
    pub confirmations: i64,
    pub blockhash: String,
    pub blockindex: i64,
    pub blocktime: i64,
    pub txid: String,
    pub walletconflicts: Vec<String>,
    pub time: i64,
    pub timereceived: i64,
    pub bip125_replaceable: Option<String>,
    pub label: String,
    pub involves_watchonly: Option<bool>,
}

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub address: String,
}

#[derive(Debug)]
pub struct Res {
    pub transactions: i64,
    pub total: i64,
}

#[derive(Debug)]
pub struct Tx {
    pub count: i64,
    pub vout: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Obj {
    total_count: i64,
    incomplete_results: bool,
}
