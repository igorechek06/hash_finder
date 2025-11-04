use clap::Parser;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use sha2::{Digest, Sha256};

use crate::args::Args;

mod args;

fn ends_with_hex_zeros(zeros: usize, hash: &[u8]) -> bool {
    let full_bytes = zeros / 2;

    // 1. Check if the bytes at the end are zeros. Fail if not.
    if hash[hash.len() - full_bytes..].iter().any(|b| *b != 0) {
        return false;
    }

    // 2. If zeros is odd, also check the half byte. Fail if not zero.
    if zeros % 2 != 0 && hash[hash.len() - full_bytes - 1] & 0x0F != 0 {
        return false;
    }

    true
}

fn main() {
    let args = Args::parse();

    // There is no need to use BigInt, since iterating through all u128 would take billions of years
    (1..=u128::MAX)
        .into_par_iter()
        .map(|n| (n, Sha256::digest(n.to_le_bytes())))
        .filter(|(_, hash)| ends_with_hex_zeros(args.zeros as usize, hash))
        .take_any(args.results as usize)
        .for_each(|(n, hash)| println!("{hash:x} - {n}"));
}
