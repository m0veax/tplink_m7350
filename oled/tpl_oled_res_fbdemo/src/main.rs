use std::fs::File;
use std::io::Write;

const DEV: &str = "/sys/class/display/oled/oled_buffer";

fn main() -> std::io::Result<()> {
    let mut f = File::create(DEV)?;
    let mut b = vec![0; 2052];
    b[2] = 128;
    b[3] = 128;

    for i in 4..2050 {
        if i % 3 == 0 {
            b[i] = 0b11110000;
        }
        if i % 4 == 0 {
            b[i] = 0b01010101;
        }
        if i % 5 == 0 {
            b[i] = 0b00001111;
        }
        if i % 7 == 0 {
            b[i] = 0b10101010;
        }
    }

    f.write_all(&b)?;

    Ok(())
}
