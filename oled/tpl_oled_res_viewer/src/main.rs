use clap::Parser;
use std::fs::{self, File};
use std::io::{self, prelude::*, SeekFrom};

/// Parse TP-Link oled_res file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {

    /// File to read
    #[arg(index = 1)]
    file: String,
}


const width: usize = 9;
const height: usize = 16;

type Row = [bool;width];

type Bitmap = [Row;height];

fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = args.file;

	/*
    let mut f = File::open(file)?;
    let m = f.metadata().unwrap();
    let size = m.len();
	*/

	let sprite: Vec<u8> = fs::read(file).unwrap();

	let mut bitmap: Bitmap = [
		[false;width];height
	];

	for row in 0..height {

		for col in 0..width {
			let byte =  (row * width) / 8;
			let bit = (((row) * width) + col) % 8; // le magic

			println!("byte {byte:03}, bit {bit}, row {row:02}, column {col:02}");

			bitmap[row][col] = ((sprite[byte] >> (8 - bit)) & 1) == 1;
		}
	}

	for y in bitmap.into_iter() {


		for x in y.into_iter() {

			print!("{}", if x {
				"#"
			} else {
				"O"
			});
		}
		println!("");
	}

	Ok(())
	

}
