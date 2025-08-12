use anyhow::bail;
use colored::Colorize;
use diff::Diff;
use redb::{ReadableDatabase, ReadableTable};

use crate::{TABLE, tx::Transaction};

pub fn compare(db1: redb::Database, db2: redb::Database, quiet: bool) -> anyhow::Result<()> {
    let read_txn1 = db1.begin_read()?;
    let table1 = read_txn1.open_table(TABLE)?;

    let read_txn2 = db2.begin_read()?;
    let table2 = read_txn2.open_table(TABLE)?;

    let mut err = false;
    for (a, b) in table1.iter()?.zip(table2.iter()?) {
        let a = a?.1.value();
        let b = b?.1.value();

        let tx1: Transaction = serde_json::from_str(&a)?;
        let tx2: Transaction = serde_json::from_str(&b)?;

        if tx1 != tx2 {
            err = true;
            if !quiet {
                eprintln!("{}: {:#?}", "Diff".red(), tx1.diff(&tx2));
            }
        } else {
            if !quiet {
                eprintln!("{}: {:#?}", "Same".green(), tx1);
            }
        }
    }

    if err {
        bail!("Mismatch detected!")
    }

    Ok(())
}

pub fn compare_with_vec(
    db1: redb::Database,
    v: &Vec<Transaction>,
    quiet: bool,
) -> anyhow::Result<()> {
    let read_txn1 = db1.begin_read()?;
    let table1 = read_txn1.open_table(TABLE)?;

    let mut err = false;
    for (a, tx2) in table1.iter()?.zip(v.iter()) {
        let a = a?.1.value();

        let tx1: Transaction = serde_json::from_str(&a)?;

        if tx1 != *tx2 {
            err = true;
            if !quiet {
                eprintln!("{}: {:#?}", "Diff".red(), tx1.diff(&tx2));
            }
        } else {
            if !quiet {
                eprintln!("{}: {:#?}", "Same".green(), tx1);
            }
        }
    }

    if err {
        bail!("Mismatch detected!")
    }

    Ok(())
}
