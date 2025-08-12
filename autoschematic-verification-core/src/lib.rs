pub mod tx;
pub mod crosscheck;
pub mod sequence;
use redb::{Database, Error, ReadableTable, TableDefinition};

pub const TABLE: TableDefinition<u128, String> = TableDefinition::new("transactions");