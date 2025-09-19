use anyhow::Result;
use aetherium_bloom;

mod simple;

fn main() -> Result<()> {
    simple::run()
}