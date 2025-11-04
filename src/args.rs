use clap::{Parser, value_parser};

/// Program that finds SHA256 hashes with trailing zeros
#[derive(Parser)]
pub struct Args {
    /// Number of trailing zeros
    #[arg(short = 'N', default_value = "3", value_parser = value_parser!(u8).range(1..=64))]
    pub zeros: u8,

    /// Number of result hashes
    #[arg(short = 'F', default_value = "1", value_parser = value_parser!(u32).range(1..))]
    pub results: u32,
}
