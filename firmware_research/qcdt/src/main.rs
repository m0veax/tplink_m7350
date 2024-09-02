use clap::Parser;
use std::fs::File;
use std::io::{self, prelude::*, SeekFrom};
use zerocopy::FromBytes;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

mod util;

/// Parse QCDT file
///
/// See also:
/// - https://wiki.postmarketos.org/wiki/QCDT
/// - https://wiki.postmarketos.org/wiki/Dtbtool
/// - https://github.com/loicpoulain/skales
/// - https://github.com/rajatgupta1998/android_tools_system_dtbTool
/// - https://github.com/s0be/dtimgextract
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
    #[arg(index = 2, default_value = "./dtbs")]
    dest: String,
}

/// Based on s0be/dtimgextract
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C)]
struct QcdtHeader {
    magic: [u8; 4],
    version: u32,
    entry_count: u32,
}

/// Based on s0be/dtimgextract
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C)]
struct QcdtEntryV1 {
    platform_id: u32,
    variant_id: u32,
    subtype_id: u32,
    offset: u32,
    size: u32,
}

/// Based on s0be/dtimgextract
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C)]
struct QcdtEntryV2 {
    platform_id: u32,
    variant_id: u32,
    sec_rev: u32,
    unknown: u32,
    offset: u32,
    size: u32,
}

/// Based on s0be/dtimgextract
#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C)]
struct QcdtEntryV3 {
    platform_id: u32,
    variant_id: u32,
    subtype_id: u32,
    sec_rev: u32,
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
    entries: &'a Vec<QcdtEntryV1>,
}

const DT_MAGIC: u32 = 0xd00d_feed;

fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = args.file;
    let dest = args.dest;
    let print = args.print;
    let extract = args.extract;

    let mut f = File::open(file.clone())?;
    let m = f.metadata().unwrap();
    let size = m.len();

    println!("Parsing {file} ({size} bytes)");

    let buf = &mut [0u8; 12];
    let _ = f.read(buf);
    let header = QcdtHeader::read_from_prefix(buf).unwrap();
    println!("{header:#010x?}");

    let mut entries: Vec<QcdtEntryV1> = vec![];
    for _ in 0..header.entry_count {
        let buf = &mut [0u8; 20];
        let _ = f.read(buf);
        let entry = QcdtEntryV1::read_from_prefix(buf).unwrap();
        entries.push(entry);
    }

    let qcdt: Qcdt = Qcdt {
        header,
        entries: &entries,
    };

    for (i, e) in qcdt.entries.iter().enumerate() {
        let QcdtEntryV1 {
            platform_id,
            variant_id,
            subtype_id,
            offset,
            size,
        } = e;

        f.seek(SeekFrom::Start(*offset as u64)).unwrap();

        // now we should find a DTB
        let buf = &mut [0u8; 4];
        let _ = f.read(buf);

        let first4 = u32::from_be_bytes(*buf);
        if first4 != DT_MAGIC {
            panic!("    {first4:08x} @ 0x{offset:08x} != 0x{DT_MAGIC:08}");
        }

        let buf = &mut [0u8; 4];
        let _ = f.read(buf);
        let sz = u32::from_be_bytes(*buf);

        if print {
            println!();
            println!(
            "Entry {i:02}: platform {platform_id}, variant {variant_id}, subtype {subtype_id:x}"
        );
            println!("  offset {offset:08x}, size {size:08x}");
            println!("  DTB is really {sz} (0x{sz}) bytes");
        }

        if extract {
            let dtb_name = format!("{dest}/{i:02}@{offset:08x}.dtb");
            todo!("extract DTB to {dtb_name}...");
        }
    }

    Ok(())
}
