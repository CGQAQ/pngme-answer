//! # Requirements
//! You should have four subcommands each with their own set of parameters.
//!
//! Encode
//! - File path
//! - Chunk type
//! - Message
//! - Output file path (Optional)
//! Decode
//! - File path
//! - Chunk type
//! Remove
//! - File path
//! - Chunk type
//! Print
//! - File path
use clap::{Parser, Subcommand};

#[clap(version = "1.0", author = "CGQAQ")]
#[derive(Parser, Debug)]
pub struct Opts {
    #[command(subcommand)]
    pub command: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Encode {
        #[clap(short, long)]
        file: String,
        #[clap(short, long)]
        chunk_type: String,
        #[clap(short, long)]
        message: String,
        #[clap(short, long)]
        output: Option<String>,
    },
    Decode {
        #[clap(short, long)]
        file: String,
        #[clap(short, long)]
        chunk_type: String,
    },
    Remove {
        #[clap(short, long)]
        file: String,
        #[clap(short, long)]
        chunk_type: String,
    },
    Print {
        #[clap(short, long)]
        file: String,
    },
}
