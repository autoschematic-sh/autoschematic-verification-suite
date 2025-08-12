use std::{path::PathBuf, process::Command};

use itertools::Itertools;
use redb::{ReadableDatabase, ReadableTable};
use serde::{Deserialize, Serialize};

use crate::{TABLE, crosscheck, tx::Transaction};

#[derive(Default, Serialize, Deserialize)]
pub struct Sequence {
    commands: Vec<Vec<String>>,
    tx_stores: Vec<String>,
    expected_txs: Vec<Transaction>,
}

impl Sequence {
    pub fn run(&self) -> anyhow::Result<()> {
        for tx_store in &self.tx_stores {
            let tx_store = PathBuf::from(tx_store);

            if tx_store.is_file() {
                std::fs::remove_file(tx_store)?;
            }
        }

        for command in &self.commands {
            Command::new(command.first().unwrap())
                .args(command.into_iter().skip(1))
                .status()?;
        }

        for tx_store in &self.tx_stores {
            let db1 = redb::Database::open(tx_store)?;
            crosscheck::compare_with_vec(db1, &self.expected_txs, false)?;
        }

        Ok(())
    }

    pub fn record(&mut self) -> anyhow::Result<()> {
        for tx_store in &self.tx_stores {
            let tx_store = PathBuf::from(tx_store);

            if tx_store.is_file() {
                std::fs::remove_file(tx_store)?;
            }
        }

        for command in &self.commands {
            Command::new(command.first().unwrap())
                .args(command.into_iter().skip(1))
                .status()?;
        }

        for (store1, store2) in self.tx_stores.iter().tuple_windows() {
            let db1 = redb::Database::open(store1)?;
            let db2 = redb::Database::open(store2)?;
            crosscheck::compare(db1, db2, false)?;
        }

        self.expected_txs = Vec::new();
        let db1 = redb::Database::open(self.tx_stores.first().unwrap())?;
        let read_txn1 = db1.begin_read()?;
        let table1 = read_txn1.open_table(TABLE)?;

        for tx in table1.iter()? {
            let tx = tx?.1.value();
            let tx: Transaction = serde_json::from_str(&tx)?;
            self.expected_txs.push(tx);
        }

        Ok(())
    }
}
