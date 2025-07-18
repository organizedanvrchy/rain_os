// src/vga_buffer.rs

use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

// Define Global WRITER using lazy_static and spinlocks
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// Disable compiler warnings for each unused variant
#[allow(dead_code)]
// Enable copy semantics for the type and make it printable and comparable
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Store each enum variant as a u8, although 4 bits is enough
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

// Represent full color code that specifies foreground and background color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Ensure ColorCode has the same data layout as a u8
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// Structures to represent a screen character and the text buffer 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Ensures struct fields are laid out exactly like in a C struct 
// (guarantees correct field ordering)
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// Writer type to write to screen
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            // if byte is newline byte call new_line method and print nothing
            b'\n' => self.new_line(),
            // Print other bytes
            byte => {
                // If current line is full, call new_line
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                
                // Write new ScreenChar to the buffer at current position
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                // Advance column position
                self.column_position += 1;
            }
        }
    }
    
    // To print whole strings, convert to bytes and print one-by-one
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
    
    // Iterate over all screen characters and move each one row up
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row-1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }
    
    // Clears row by overwriting all characters with space characters
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

// Support printing different types (such as int and float)
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// print! Macro with rules
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

// println! Macro with rules
#[macro_export] // Makes macro available to whole crate and external crates
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// Print given formatted string to the VGA text buffer
// through global `WRITER` instance,
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

// **Testing VGA Buffer**

// Test println works without panicking
#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

// Test is panic occurs even if many lines are printable and shifted off screen
#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

// Test to verify that printed lines really appear on the screen
#[test_case]
fn test_println_output() {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}

