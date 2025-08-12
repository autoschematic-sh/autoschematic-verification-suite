use clap::{arg, command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "autoschematic-testbench")]
pub struct AutoschematicTestBenchCommand {
    #[command(subcommand)]
    pub command: AutoschematicTestBenchSubcommand,
}

#[derive(Subcommand, Debug)]
pub enum AutoschematicTestBenchSubcommand {
    /// Create a blank sequence file
    Init {
        #[arg(short, long)]
        sequence: String,
    },
    /// Run a test sequence and verify that the resulting transactions matched the sequence.
    Run {
        #[arg(short, long)]
        sequence: String,
    },
    /// Run a test sequence and save its transactions back to that same sequence file.
    Record {
        #[arg(short, long)]
        sequence: String,
    },
}
