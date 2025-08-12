use std::time::SystemTime;

use diff::Diff;
use serde::{Deserialize, Serialize};

use crate::TABLE;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, PartialEq, Diff)]
#[diff(attr(
    #[derive(Debug, PartialEq)]
))]
pub struct Transaction {
    pub kind: String,
    pub params: Vec<String>,
}

impl Transaction {
    pub fn write(&self, db: &redb::Database) -> anyhow::Result<()> {
        let write_txn = db.begin_write()?;

        {
            let duration_since_epoch = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            let timestamp_nanos = duration_since_epoch.as_nanos();
            let mut table = write_txn.open_table(TABLE)?;

            let tx_s = serde_json::to_string(self)?;

            table.insert(timestamp_nanos, &tx_s)?;
        }

        write_txn.commit()?;

        Ok(())
    }
}
