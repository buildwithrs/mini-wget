use std::{env, path::PathBuf};

use mini_wget::Downloader;

fn main() -> anyhow::Result<()> {
    println!("A Mini Wget write by Rust");

    let args: Vec<String> = env::args().collect();

    let dest = PathBuf::from(&args[2]);

    let dl = Downloader::new(&args[1], dest)?;
    dl.download()?;
    Ok(())
}
