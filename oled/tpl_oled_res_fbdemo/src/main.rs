use std::os::unix::fs::FileExt;
use std::process::Command;
use std::time::Duration;
use std::{fs::File, thread::sleep};

use embedded_graphics::pixelcolor::raw::ToBytes;
use embedded_graphics::{
    mono_font::{ascii::FONT_5X8 as FONT, MonoTextStyle},
    pixelcolor::Rgb565 as COLOR,
    prelude::*,
    text::Text,
};

#[cfg(any(target_arch = "arm"))]
const DEV: &str = "/sys/class/display/oled/oled_buffer";
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const DEV: &str = "oled_buffer";

// Full path, so that we can `cpu` without namespace issues.
#[cfg(any(target_arch = "arm"))]
const IP_CMD: &str = "/bbin/ip";
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const IP_CMD: &str = "ip";

const WRITE_TO_FILE: bool = true;
const WRITE_TO_TERM: bool = true;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

const STRIDE: u32 = WIDTH * 2;

// NOTE: 16 bits per pixel
const FB_SIZE: usize = (STRIDE * HEIGHT) as usize;

enum PixelColor {
    Pink,
    Red,
    Blue,
    Green,
}

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
        // We need to prepend the coordinates (bytes 0+1) and dimensions (2+3).
        #[cfg(any(target_arch = "arm"))]
        {
            const CHUNK_SIZE: usize = 2048;
            const CHUNK_COUNT: usize = FB_SIZE / CHUNK_SIZE;
            const CHUNK_HEIGHT: u8 = (HEIGHT / CHUNK_COUNT as u32) as u8;
            for c in 0..CHUNK_COUNT {
                let x = 0u8;
                let y = c as u8 * CHUNK_HEIGHT;
                let coords_and_dims = &[x, y, WIDTH as u8, CHUNK_HEIGHT];
                let offset = c * CHUNK_SIZE;
                let chunk = &self.framebuffer[offset..(offset + CHUNK_SIZE)];
                let data = &[coords_and_dims, chunk].concat();
                self.dev.write_all_at(data, 0).unwrap();
            }
        }
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if WRITE_TO_FILE {
                self.dev.write_all_at(&self.framebuffer, 0).unwrap();
            }
            if WRITE_TO_TERM {
                for line in self.framebuffer.chunks(WIDTH as usize) {
                    for byte in line {
                        print!("{}", if byte & 0xf == 0 { " " } else { "█" });
                    }
                    println!();
                }
            }
        }
    }

    pub fn draw_lines(&mut self, color: PixelColor, o: usize) {
        // two bytes per pixel
        let (b1, b2) = match color {
            PixelColor::Pink => (0b11111000, 0b00011111),
            PixelColor::Red => (0b11111000, 0),
            PixelColor::Green => (0b00000111, 0b11100000),
            PixelColor::Blue => (0, 0b00011111),
        };

        for y in 0..HEIGHT as usize {
            if y % o == 0 {
                let l = y * STRIDE as usize;
                for x in 0..WIDTH as usize {
                    self.framebuffer[l + x * 2] = b1;
                    self.framebuffer[l + x * 2 + 1] = b2;
                }
            }
        }
    }

    pub fn draw_pattern(&mut self, o: usize) {
        for i in 0..FB_SIZE {
            if i % (o + 3) == 0 {
                self.framebuffer[i] = 0b11110000;
            }
            if i % (o + 4) == 0 {
                self.framebuffer[i] = 0b01010101;
            }
            if i % (o + 5) == 0 {
                self.framebuffer[i] = 0b00001111;
            }
            if i % (o + 7) == 0 {
                self.framebuffer[i] = 0b10101010;
            }
        }
    }

    pub fn clear(&mut self) {
        let b = [0u8; FB_SIZE];
        self.framebuffer = b;
    }
}

impl DrawTarget for Display {
    type Color = COLOR;
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
                let l = y * STRIDE;
                let i = (l + x * 2) as usize;
                let pixel = color.to_be_bytes();
                self.framebuffer[i] = pixel[0];
                self.framebuffer[i + 1] = pixel[1];
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

fn print_ip_a(display: &mut Display) {
    display.clear();
    let ipa = Command::new(IP_CMD).arg("a").output().unwrap().stdout;
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
    let out = lines.collect::<Vec<_>>().join("\n");

    let style = MonoTextStyle::new(&FONT, COLOR::GREEN);
    let text = Text::new(out.as_str(), Point::new(3, 6), style);
    text.draw(display).unwrap();
    display.flush();
}

const TEST: bool = true;

fn main() -> std::io::Result<()> {
    let mut display = Display::new(DEV);

    if TEST {
        for o in [0, 2, 1, 5, 9] {
            display.clear();
            display.draw_pattern(o);
            display.flush();
            sleep(Duration::from_millis(200));
        }

        for o in 1..3 {
            for c in [PixelColor::Red, PixelColor::Green, PixelColor::Blue] {
                sleep(Duration::from_millis(200));
                display.clear();
                display.draw_lines(c, o);
                display.flush();
            }
        }
    }

    print_ip_a(&mut display);

    Ok(())
}
