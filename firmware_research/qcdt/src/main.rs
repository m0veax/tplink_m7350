use clap::Parser;
use std::fs::File;
use std::io::{self, prelude::*, SeekFrom};
use zerocopy::FromBytes;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

/// Parse QCDT file
///
/// See also:
/// - https://wiki.postmarketos.org/wiki/QCDT
/// - https://wiki.postmarketos.org/wiki/Dtbtool
/// - https://github.com/loicpoulain/skales
/// - https://github.com/rajatgupta1998/android_tools_system_dtbTool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Print sprite metadata
    #[arg(required = false, short, long)]
    print: bool,

    /// Extract sprites to files
    #[arg(required = false, short, long)]
    extract: bool,

    /// File to read
    #[arg(index = 1)]
    file: String,

    // output directory
    #[arg(index = 2, default_value = "./res")]
    dest: String,
}

/// Based on `rajatgupta1998/android_tools_system_dtbTool` `source/dtbtool.txt`
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C)]
struct QcdtHeader {
    magic: [u8; 4],
    version: u32,
    dt_count: u32,
}

#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C)]
struct QcdtEntry {
    platform_id: u32,
    variant_id: u32,
    subtype_id: u32,
    soc_rev: u32,
    pmic0: u32,
    pmic1: u32,
    pmic2: u32,
    pmic3: u32,
    offset: u32,
    size: u32,
}

#[derive(Clone, Debug)]
#[repr(C)]
struct Qcdt<'a> {
    header: QcdtHeader,
    entries: &'a Vec<QcdtEntry>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = args.file;
    let dest = args.dest;
    let print = args.print;
    let extract = args.extract;

    println!("Parsing {file}");

    let mut f = File::open(file)?;
    let m = f.metadata().unwrap();
    let size = m.len();

    // read header
    let buf = &mut [0u8; 12];
    let _ = f.read(buf);

    let mut header = QcdtHeader::read_from_prefix(buf).unwrap();
    // apparently, this is ASCII (?) - we get 0x37 where it should be 7...
    header.dt_count -= 0x30;

    // entries ...
    let mut entries: Vec<QcdtEntry> = vec![];

    loop {
        let buf = &mut [0u8; 40];
        let _ = f.read(buf);

        let entry = QcdtEntry::read_from_prefix(buf).unwrap();
        entries.push(entry);

        if entry.size == 0 {
            break;
        }
    }

    let qcdt: Qcdt = Qcdt {
        header,
        entries: &entries,
    };

    println!("{qcdt:#010x?}");

    println!("{}", qcdt.entries.len());

    const PAGE_SIZE: u64 = 2048;
    // seek to next page boundary
    let pos = f
        .stream_position()
        .expect("Could not get current position!");
    let pad = ((pos as f64 / PAGE_SIZE as f64).round()) as u64 * PAGE_SIZE - pos;

    f.seek(SeekFrom::Current(pad as i64)).unwrap();

    // now we should find a DTB
    let buf = &mut [0u8; 4];
    let _ = f.read(buf);

    let d00dfeed = u32::from_be_bytes(*buf);
    assert!(d00dfeed == 0xd00d_feed);

    Ok(())
}
