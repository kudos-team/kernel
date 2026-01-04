use kudos::vga_buffer::{ColorCode, Color, WRITER, DEFAULT_FG, DEFAULT_BG};
use core::fmt;
use core::fmt::Write;
use x86_64::instructions::interrupts;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LogType {
    Info = 0,
    Good = 1,
    Warn = 2,
    Error = 3,
}

#[doc(hidden)]
pub fn _print_status(typ: LogType, args: fmt::Arguments) {
    let (c, col) = match typ {
        LogType::Info => (b'~', Color::Magenta),
        LogType::Good => (b'+', Color::Green),
        LogType::Warn => (b'*', Color::Yellow),
        LogType::Error => (b'-', Color::Red),
    };
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();

        writer.color_code = ColorCode::new(DEFAULT_FG, DEFAULT_BG);
        writer.write_string("[");
        writer.color_code = ColorCode::new(col, DEFAULT_BG);
        writer.write_byte(c);
        writer.color_code = ColorCode::new(DEFAULT_FG, DEFAULT_BG);
        writer.write_string("] ");

        writer.write_fmt(args).unwrap();
        writer.write_byte(b'\n');
    });
}


#[macro_export]
macro_rules! printLog {
    ($typ:expr, $($arg:tt)*) => {
        $crate::utils::fancy::_print_status($typ, format_args!($($arg)*))
    };
    ($($arg:tt)*) => {
        $crate::utils::fancy::_print_status($crate::utils::fancy::LogType::Info, format_args!($($arg)*))
    };
}

