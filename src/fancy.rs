use kudos::vga_buffer::{ColorCode, Color, WRITER, DEFAULT_FG, DEFAULT_BG};
use core::fmt;
use core::fmt::Write;
use x86_64::instructions::interrupts;

#[doc(hidden)]
pub fn _print_status(args: fmt::Arguments, c: u8, col: Color) {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();

        writer.color_code = ColorCode::new(DEFAULT_FG, DEFAULT_BG);
        writer.write_string(" [");
        writer.color_code = ColorCode::new(col, DEFAULT_BG);
        writer.write_byte(c);
        writer.color_code = ColorCode::new(DEFAULT_FG, DEFAULT_BG);
        writer.write_string("] ");

        writer.write_fmt(args).unwrap();
        writer.write_byte(b'\n');
    });
}


#[macro_export]
macro_rules! printGood {
    ($($arg:tt)*) => ($crate::fancy::_print_status(format_args!($($arg)*), b'+', kudos::vga_buffer::Color::Green));
}

