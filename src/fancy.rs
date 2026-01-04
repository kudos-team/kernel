use kudos::{print, println};
use kudos::vga_buffer::{WRITER, ColorCode, Color};

pub fn displayScreen() {
    {
        let mut writer = WRITER.lock();
        writer.color_code = ColorCode::new(Color::Black, Color::Yellow);
    }
    println!("Hello!");
}

