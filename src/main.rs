use postgres::{Client, Error, NoTls};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use tx_parser::types;
use uuid::Uuid;

fn check_duplicates(tx_map: &mut HashMap<String, types::Tx>, log: &types::Transaction) -> bool {
    let x = types::Tx { count: 0, vout: 0 };
    if !tx_map.contains_key(&log.txid) {
        tx_map.insert(log.txid.to_string(), x);
    }
    tx_map.entry(log.txid.to_string()).and_modify(|e| {
        e.count += 1;
    });

    if tx_map.get(&log.txid).unwrap().count > 1 && tx_map.get(&log.txid).unwrap().vout == log.vout {
        return true;
    }
    tx_map.entry(log.txid.to_string()).and_modify(|e| {
        e.vout = log.vout;
    });
    return false;
}

fn parse(s: &str) -> types::Transactions {
    let mut file = File::open(s).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let issues = serde_json::from_str::<types::Transactions>(&contents).unwrap();
    issues
}

fn connect_db() -> Result<Client, Error> {
    let mut client = Client::connect(
        "host=localhost user=postgres password=password dbname=rust",
        NoTls,
    )?;
    client.batch_execute(r#"CREATE EXTENSION IF NOT EXISTS "uuid-ossp""#)?;
    client.batch_execute(
        r#"
        CREATE TABLE IF NOT EXISTS transactions (id TEXT PRIMARY KEY, account VARCHAR(255), address VARCHAR(255), amount DOUBLE PRECISION, category VARCHAR(255), confirmations BIGINT, vout BIGINT)
        "#,
    )?;
    Ok(client)
}

fn main() -> Result<(), Error> {
    let mut client = connect_db().unwrap();

    let mut user_address: Vec<String> = vec![];
    let wesley = types::User {
        name: "Wesley Crusher".to_string(),
        address: "mvd6qFeVkqH6MNAS2Y2cLifbdaX5XUkbZJ".to_string(),
    };
    let leonard = types::User {
        name: "Leonard McCoy".to_string(),
        address: "mmFFG4jqAtw9MoCC88hw5FNfreQWuEHADp".to_string(),
    };
    let jonathan = types::User {
        name: "Jonathan Archer".to_string(),
        address: "mzzg8fvHXydKs8j9D2a8t7KpSXpGgAnk4n".to_string(),
    };
    let jadzia = types::User {
        name: "Jadzia Dax".to_string(),
        address: "2N1SP7r92ZZJvYKG2oNtzPwYnzw62up7mTo".to_string(),
    };
    let montgomery = types::User {
        name: "Montgomery Scott".to_string(),
        address: "mutrAf4usv3HKNdpLwVD4ow2oLArL6Rez8".to_string(),
    };
    let james = types::User {
        name: "James T. Kirk".to_string(),
        address: "miTHhiX3iFhVnAEecLjybxvV5g8mKYTtnM".to_string(),
    };
    let spock = types::User {
        name: "Spock".to_string(),
        address: "mvcyJMiAcSXKAEsQxbW9TYZ369rsMG6rVV".to_string(),
    };
    let users = vec![wesley, leonard, jonathan, jadzia, montgomery, james, spock];
    let mut trans: HashMap<String, types::Tx> = HashMap::new();
    let files = vec!["src/tx1.json", "src/tx2.json"];
    println!("Running...");
    for x in files.iter() {
        let txs = parse(x);
        for tx in txs.transactions.iter() {
            let ok = check_duplicates(&mut trans, tx);
            if ok {
                continue;
            }
            let my_uuid = Uuid::new_v4().to_string();
            client.execute(
                "INSERT INTO transactions (id, account, address, amount, category, confirmations, vout) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                &[&my_uuid,
                &tx.account,
                &tx.address,
                &tx.amount,
                &tx.category,
                &tx.confirmations,
                &tx.vout,],
            )?;
        }
    }
    for user in users.iter() {
        user_address.push(user.address.clone());
        for row in client.query("SELECT count(transactions) as Transactions, sum(amount) as Total from transactions where address = $1 AND confirmations > $2",
    &[&user.address, &i64::from(5)])? {
            println!("Deposited for {}: count={} sum={}", user.name, row.get::<usize, i64>(0), row.get::<usize, f64>(1));
        }
    }

    Ok(())
}
