use std::os::unix::fs::FileExt;
use std::process::Command;
use std::time::Duration;
use std::{fs::File, thread::sleep};

use embedded_graphics::pixelcolor::raw::ToBytes;
use embedded_graphics::primitives::{PrimitiveStyleBuilder, Rectangle};
use embedded_graphics::{
    image::{Image, ImageRaw, ImageRawLE as ImageFormat},
    mono_font::{ascii::FONT_5X8 as FONT, MonoTextStyle},
    pixelcolor::Rgb565 as COLOR,
    prelude::*,
    text::Text,
};
use evdev::{Device, KeyCode};

const DEMO_ANIMATION: bool = false;
const HACK_THE_PLANET: bool = true;

// Verbose debug prints.
const VERBOSE: bool = false;

#[cfg(any(target_arch = "arm"))]
const DEV: &str = "/sys/class/display/oled/oled_buffer";
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const DEV: &str = "oled_buffer";

// Full path, so that we can `cpu` without namespace issues.
#[cfg(any(target_arch = "arm"))]
const IP_CMD: &str = "/bbin/ip";
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const IP_CMD: &str = "ip";

// Debug options for local use on your x86 dev machine
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const WRITE_TO_FILE: bool = true;
// NOTE: This works for b/w output, not very useful with colors, only for text.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const WRITE_TO_TERM: bool = true;

// These are driver specifics for the Arm target.
// The current driver is using a sysfs file for the framebuffer, no `/dev/fbX`,
// so only PAGE_SIZE-1 (4095) bytes can be sent to it in one shot (a terminating
// null-byte is added by sysfs), so split into chunks.
// https://docs.kernel.org/filesystems/sysfs.html
#[cfg(any(target_arch = "arm"))]
const CHUNK_SIZE: usize = 2048;
#[cfg(any(target_arch = "arm"))]
const CHUNK_COUNT: usize = FB_SIZE / CHUNK_SIZE;
#[cfg(any(target_arch = "arm"))]
const CHUNK_HEIGHT: u8 = (HEIGHT / CHUNK_COUNT as u32) as u8;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;

const STRIDE: u32 = WIDTH * 2;

// NOTE: These are raw images, so they do not contain information on dimensions.
// By convetion, they must be full width.
const IMAGE_FLAG: &[u8] = include_bytes!("pride-flag-chaos.raw");
const IMAGE_FERRIS: &[u8] = include_bytes!("ferris_128.raw");

// NOTE: 16 bits per pixel
const FB_SIZE: usize = (STRIDE * HEIGHT) as usize;

// RGB565
#[repr(u16)]
enum PixelColor {
    Red = 0b11111000_00000000,
    Green = 0b00000111_11100000,
    Blue = 0b00000000_00011111,
    Pink = 0b11111000_00011111,
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

    /// Updates the display from our internal framebuffer.
    pub fn flush(&mut self) {
        // We need to prepend the coordinates (bytes 0+1) and dimensions (2+3).
        #[cfg(any(target_arch = "arm"))]
        for c in 0..CHUNK_COUNT {
            let y = c as u8 * CHUNK_HEIGHT;
            let coords_and_dims = &[0, y, WIDTH as u8, CHUNK_HEIGHT];
            let offset = c * CHUNK_SIZE;
            let chunk = &self.framebuffer[offset..(offset + CHUNK_SIZE)];
            let data = &[coords_and_dims, chunk].concat();
            self.dev.write_all_at(data, 0).unwrap();
        }
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if WRITE_TO_FILE {
                self.dev.write_all_at(&self.framebuffer, 0).unwrap();
            }
            if WRITE_TO_TERM {
                for line in self.framebuffer.chunks(STRIDE as usize) {
                    for byte in line {
                        print!("{}", if byte & 0xf0 > 0xf { " " } else { "█" });
                    }
                    println!();
                }
            }
        }
    }

    pub fn draw_lines(&mut self, color: PixelColor, gap: usize) {
        // two bytes per pixel
        let c = color as u16;
        for y in 0..HEIGHT as usize {
            if gap == 0 || y % gap == 0 {
                let l = y * STRIDE as usize;
                for x in 0..WIDTH as usize {
                    self.framebuffer[l + x * 2] = (c >> 8) as u8;
                    self.framebuffer[l + x * 2 + 1] = c as u8;
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

    let style = MonoTextStyle::new(&FONT, COLOR::CSS_GREEN_YELLOW);
    let text = Text::new(out.as_str(), Point::new(3, 6), style);
    text.draw(display).unwrap();
}

// https://docs.rs/embedded-graphics/latest/embedded_graphics/image/index.html
fn print_image(display: &mut Display, img: &[u8]) {
    let sprite: ImageFormat<COLOR> = ImageRaw::new(img, WIDTH);
    let image = Image::new(&sprite, Point::new(0, 20));
    image.draw(display).unwrap();
}

enum BackgroundImage {
    Ferris,
    Flag,
}

// Print the chosen background image.
fn print_bg(display: &mut Display, bg_img: &BackgroundImage) {
    let img = match bg_img {
        BackgroundImage::Ferris => IMAGE_FERRIS,
        BackgroundImage::Flag => IMAGE_FLAG,
    };
    print_image(display, &img);
}

const MAX_OPTION: usize = 4;

fn print_menu(display: &mut Display, sel: usize) {
    let blank = PrimitiveStyleBuilder::new()
        .fill_color(COLOR::BLACK)
        .build();
    let selected = PrimitiveStyleBuilder::new()
        .stroke_color(COLOR::CSS_LIME_GREEN)
        .stroke_width(1)
        .fill_color(COLOR::BLACK)
        .build();
    let label = MonoTextStyle::new(&FONT, COLOR::CSS_GREEN_YELLOW);

    for i in 0..MAX_OPTION {
        let s = if sel == i { selected } else { blank };
        let o = i as i32 * 20;
        Rectangle::new(Point::new(28, 23 + o), Size::new(60, 15))
            .into_styled(s)
            .draw(display)
            .unwrap();
        let l = match i {
            0 => "pattern 1",
            1 => "pattern 2",
            2 => "change bg",
            3 => "   DEMO  ",
            _ => unreachable!(),
        };
        let text = Text::new(l, Point::new(35, 32 + o), label);
        text.draw(display).unwrap();
    }
}

fn play_demo(display: &mut Display) {
    for pattern in [0, 2, 1, 5, 9] {
        display.clear();
        display.draw_pattern(pattern);
        display.flush();
        sleep(Duration::from_millis(200));
    }

    for gap in [0, 2, 5] {
        for color in [
            PixelColor::Red,
            PixelColor::Green,
            PixelColor::Blue,
            PixelColor::Pink,
        ] {
            display.clear();
            display.draw_lines(color, gap);
            display.flush();
            sleep(Duration::from_millis(100));
        }
    }
}

/// Listen to keyboard events and act accordingly.
/// The device has two buttons, but they currently end up on different evdevs.
fn enter_loop(display: &mut Display) -> ! {
    let dev0 = Device::open("/dev/input/event0").unwrap();
    let dev1 = Device::open("/dev/input/event1").unwrap();

    let mut sel_opt = 0;
    let mut show = true;
    let mut sel_img = BackgroundImage::Flag;

    loop {
        let s = dev0.get_key_state().unwrap();
        if s.contains(KeyCode::KEY_POWER) {
            if VERBOSE {
                println!("POWER!");
            }
            display.clear();
            match sel_opt {
                0 | 1 => {
                    if show {
                        display.draw_pattern(sel_opt);
                        // Show the menu next time.
                        show = false;
                    } else {
                        print_bg(display, &sel_img);
                        print_menu(display, sel_opt);
                        // Show the pattern again next time.
                        show = true;
                    }
                }
                2 => {
                    sel_img = match sel_img {
                        BackgroundImage::Ferris => BackgroundImage::Flag,
                        BackgroundImage::Flag => BackgroundImage::Ferris,
                    };
                    print_bg(display, &sel_img);
                    print_menu(display, sel_opt);
                }
                3 => {
                    play_demo(display);
                    display.clear();
                    print_bg(display, &sel_img);
                    print_menu(display, sel_opt);
                }
                _ => unreachable!(),
            }
            display.flush();
        }
        let s = dev1.get_key_state().unwrap();
        if s.contains(KeyCode::KEY_UP) {
            if VERBOSE {
                println!("UP!");
            }
            sel_opt += 1;
            if sel_opt == MAX_OPTION {
                sel_opt = 0;
            }
            // Next time a pattern is selected, show it.
            show = true;
            display.clear();
            print_bg(display, &sel_img);
            print_menu(display, sel_opt);
            display.flush();
        }
        sleep(Duration::from_millis(60));
    }
}

fn main() {
    let mut display = Display::new(DEV);

    if DEMO_ANIMATION {
        play_demo(&mut display);
    }

    if HACK_THE_PLANET {
        display.clear();
        if cfg!(any(target_arch = "arm")) {
            print_image(&mut display, IMAGE_FLAG);
        }
        print_ip_a(&mut display);
        display.flush();
        sleep(Duration::from_millis(600));
    }

    display.clear();
    print_image(&mut display, IMAGE_FLAG);
    print_menu(&mut display, 0);
    display.flush();

    if cfg!(any(target_arch = "arm")) {
        enter_loop(&mut display);
    }
}
