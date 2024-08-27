use clap::Parser;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use zerocopy::FromBytes;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C)]
struct SpriteMeta {
    sprite_id: u16,
    _1: u16,
    _2: u16,
    _3: u16,
    width: u16,
    height: u16,
    _4: u16,
    _5: u16,
    _6: u16,
    _7: u16,
    _8: u16,
    _9: u16,
    _10: u16,
    _11: u16,
}

/// Parse TP-Link oled_res file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to read
    #[arg(index = 1)]
    file: String,
}

const META_SIZE: usize = 28;

fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = args.file;

    println!("Scanning {file} for entries");

    let mut f = File::open(file)?;
    let m = f.metadata().unwrap();
    let size = m.len();

    let buf = &mut [0u8; 2];
    let _ = f.read(buf);
    let entries = u16::from_le_bytes(*buf);

    println!("Size: {size}, entries: {entries:?}");

    let mut pos = 2;
    let mut count = 0;
    for _ in 0..entries {
        f.seek(SeekFrom::Start(pos))?;
        let buf = &mut [0u8; META_SIZE];
        let bytes_read = f.read(buf).unwrap();

        // This should not happen if the number of entries is correct.
        // You never know.
        if bytes_read == 0 {
            break;
        }

        let sm = SpriteMeta::read_from_prefix(buf).unwrap();
        let id = sm.sprite_id;
        let len = ((sm.width * sm.height) as f64 / 8.0).ceil();
        print!("{id} {len}; ");

        pos += (META_SIZE + len as usize) as u64;
        count += 1;
    }

    println!();
    println!("{count}/{entries} entries parsed");

    Ok(())
}
