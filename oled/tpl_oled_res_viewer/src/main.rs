use clap::Parser;
use std::{fs, io};

/// Parse TP-Link oled_res file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to read
    #[arg(index = 1)]
    file: String,

    #[arg(index = 2)]
    width: usize,

    #[arg(index = 3)]
    height: usize,
}

type Row = Vec<bool>;
type Bitmap = Vec<Row>;

fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = args.file;
    let width = args.width;
    let height = args.height;

    // TODO: Should we read image dimensions from file name?
    let f = fs::File::open(file.clone())?;
    let m = f.metadata().unwrap();
    let size = m.len() as usize;
    if width * height / 8 > size {
        panic!("File is {size} bytes, {width}x{height} won't fit!");
    }

    let sprite: Vec<u8> = fs::read(file).unwrap();

    let mut bitmap: Bitmap = vec![Row::new(); height];

    for (row, r) in bitmap.iter_mut().enumerate() {
        *r = vec![false; width];
        for (col, e) in r.iter_mut().enumerate() {
            let pos = row * width + col;
            let byte = pos / 8;
            let bit = 7 - (pos % 8);
            let b = (sprite[byte] >> bit) & 1;
            *e = b == 1;
        }
    }

    print!("┏");
    for _ in 0..width {
        print!("━");
    }
    println!("┓");
    for y in bitmap.into_iter() {
        print!("┃");
        for x in y.into_iter() {
            let c = if x { " " } else { "█" };
            print!("{c}");
        }
        println!("┃");
    }
    print!("┗");
    for _ in 0..width {
        print!("━");
    }
    println!("┛");

    Ok(())
}
