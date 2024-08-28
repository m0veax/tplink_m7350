use clap::Parser;
use std::fs::{self, File};
use std::io::{self, prelude::*, SeekFrom};
use zerocopy::FromBytes;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(AsBytes, FromBytes, FromZeroes, Clone, Copy, Debug)]
#[repr(C)]
struct SpriteMeta {
    sprite_id: u16,
    _1: u16, // always 0
    _2: u16,
    _3: u16,
    width: u16,
    height: u16,
    _4: u16, // mostly 1 when sprite is full screen
    _5: u16,
    _6: u16,
    _junk: [u16; 5], // always 0
}

/// Parse TP-Link oled_res file
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

const META_SIZE: usize = 28;

fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = args.file;
    let dest = args.dest;
    let print = args.print;
    let extract = args.extract;

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

    if extract {
        fs::create_dir_all(&dest).unwrap();
    }

    if print {
        println!();
        println!("  id   offset   ?    ?  width*height ?    ?    ?");
        println!();
    }

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
        let width = sm.width;
        let height = sm.height;

        let len = ((width * height) as f64 / 8.0).ceil() as usize;
        pos += META_SIZE as u64;

        if print {
            println!(
                "{id:05} @{pos:06}: {:03}  {:03}   {width:03}*{height:03}    {}  {:04}  {:03}",
                sm._2, sm._3, sm._4, sm._5, sm._6
            );
        }

        if extract {
            let mut buf = vec![0; len];
            let buf = buf.as_mut_slice();
            f.read_exact(buf).unwrap();

            let mut filename = format!("{id}_{width}_{height}");
            filename.push_str(".res");

            let path = std::path::Path::new(&dest).join(filename);
            let mut output = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)
                .unwrap();
            output.write_all(buf).unwrap();
        }

        pos += len as u64;
        count += 1;
    }

    println!();
    println!("{count}/{entries} entries parsed");

    Ok(())
}
