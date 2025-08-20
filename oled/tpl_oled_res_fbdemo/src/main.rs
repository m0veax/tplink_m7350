use std::os::unix::fs::FileExt;
use std::process::Command;
use std::time::Duration;
use std::{fs::File, thread::sleep};

use bitmap_font::{tamzen::FONT_6x12, TextStyle};
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, text::Text};

#[cfg(any(target_arch = "arm"))]
const DEV: &str = "/sys/class/display/oled/oled_buffer";
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const DEV: &str = "oled_buffer";

const WRITE_TO_FILE: bool = true;
const WRITE_TO_TERM: bool = true;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

// NOTE: 1 bit per pixel
const FB_SIZE: usize = (WIDTH / 8 * HEIGHT) as usize;

struct Display {
    framebuffer: [u8; FB_SIZE],
    dev: File,
}

impl Display {
    fn new(dev: &'static str) -> Self {
        let f = File::create(dev).unwrap();

        Self {
            framebuffer: [0xff; FB_SIZE],
            dev: f,
        }
    }

    /// Updates the display from the framebuffer.
    pub fn flush(&mut self) {
        // We need to prepend the dimensions and first two bytes are 0.
        #[cfg(any(target_arch = "arm"))]
        {
            let mut b = vec![0u8; 4];
            b[2] = WIDTH as u8;
            b[3] = HEIGHT as u8;
            b.extend(self.framebuffer.iter().cloned());
            self.dev.write_all_at(&b, 0).unwrap();
        }
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if WRITE_TO_FILE {
                self.dev.write_all_at(&self.framebuffer, 0).unwrap();
            }
            if WRITE_TO_TERM {
                for line in self.framebuffer.chunks(WIDTH as usize / 8) {
                    for byte in line {
                        for i in (0..8).rev() {
                            print!("{}", if (byte >> i) & 1 == 0 { " " } else { "█" });
                        }
                    }
                    println!();
                }
            }
        }
    }

    pub fn draw_pattern(&mut self) {
        let mut b = [0u8; FB_SIZE];

        for i in 0..FB_SIZE {
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

        self.framebuffer = b;
    }

    pub fn clear(&mut self) {
        let b = [0xffu8; FB_SIZE];
        self.framebuffer = b;
        self.flush();
    }
}

impl DrawTarget for Display {
    type Color = BinaryColor;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            // Check if the pixel coordinates are out of bounds (negative or
            // greater than (WIDTH,HEIGHT)). `DrawTarget` implementations are
            // required to discard any out of bounds pixels without erroring.

            if let Ok((x @ 0..WIDTH, y @ 0..HEIGHT)) = coord.try_into() {
                // Divide by 8 beause we have a single bit per pixel.
                let i = (x + y * HEIGHT) as usize / 8;
                // NOTE: The pixels are backwards.
                let pixel = (color as u8) << (7 - (x % 8));
                self.framebuffer[i] = self.framebuffer[i] & !pixel;
            }
        }

        Ok(())
    }
}

impl OriginDimensions for Display {
    fn size(&self) -> Size {
        Size::new(WIDTH, HEIGHT)
    }
}

fn main() -> std::io::Result<()> {
    let mut display = Display::new(DEV);

    display.draw_pattern();
    display.flush();

    sleep(Duration::from_secs(1));
    display.clear();

    let ipa = Command::new("ip").arg("a").output().unwrap().stdout;
    let ipa = str::from_utf8(&ipa).unwrap();
    // NOTE: We can only print a few characters per line.
    // The library will cut off extra pixels for us.
    // We do want to discard unnecessary characters in the beginning though.
    let lines = ipa.split("\n").into_iter().map(|l| {
        // The first 3 characters are just the index, followed a colon, a space,
        // and then the name, and details, e.g., `1: lo: <...`
        let min = l.len().min(3);
        l[min..].to_string()
    });
    let out = lines.collect::<Vec<_>>()[..10].join("\n");

    let text = Text::new(
        out.as_str(),
        Point::new(1, 4),
        TextStyle::new(&FONT_6x12, BinaryColor::On),
    );
    text.draw(&mut display).unwrap();
    display.flush();

    Ok(())
}
