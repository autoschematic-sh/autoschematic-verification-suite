pub mod cmd;
use std::process::Command;

use anyhow::Context;
use autoschematic_verification_core::{
    crosscheck,
    sequence::{self, Sequence},
};
use clap::Parser;
use ron::ser::PrettyConfig;
use tracing_subscriber::EnvFilter;

use crate::cmd::AutoschematicTestBenchCommand;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_target(true)
        .with_thread_ids(false)
        .with_line_number(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cmd = AutoschematicTestBenchCommand::parse();

    match cmd.command {
        cmd::AutoschematicTestBenchSubcommand::Init { sequence } => {
            std::fs::write(
                sequence,
                ron::ser::to_string_pretty(&Sequence::default(), PrettyConfig::default())?,
            )?;
        }
        cmd::AutoschematicTestBenchSubcommand::Run { sequence } => {
            let paths = std::fs::read_dir("./").unwrap();

            for path in paths {
                println!("Name: {}", path.unwrap().path().display())
            }
            let sequence: Sequence = ron::from_str(
                &std::fs::read_to_string(&sequence).context(format!("reading {}", sequence))?,
            )?;
            sequence.run()?;
        }
        cmd::AutoschematicTestBenchSubcommand::Record { sequence } => {
            let mut out_sequence: Sequence = ron::from_str(&std::fs::read_to_string(&sequence)?)?;
            out_sequence.record()?;
            std::fs::write(
                sequence,
                ron::ser::to_string_pretty(&out_sequence, PrettyConfig::default())?,
            )?;
        }
    }

    // match cmd.command {
    // std::fs::remove_file("testbench/equivalence/tarpc/scoreboard.redb")?;
    // std::fs::remove_file("testbench/equivalence/grpc/scoreboard.redb")?;

    // Command::new("autoschematic").args(["apply"]).status()?;

    // let db1 = redb::Database::open("testbench/equivalence/tarpc/scoreboard.redb")?;
    // let db2 = redb::Database::open("testbench/equivalence/grpc/scoreboard.redb")?;

    // crosscheck::compare(db1, db2)?;

    Ok(())
}
