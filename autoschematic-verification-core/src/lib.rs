pub mod tx;
pub mod crosscheck;
pub mod sequence;
use redb::{TableDefinition};

pub const TABLE: TableDefinition<u128, String> = TableDefinition::new("transactions");