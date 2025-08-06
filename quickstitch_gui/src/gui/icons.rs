use iced::{
    Alignment, Font,
    widget::{Text, text},
};

pub const ICON_FONT: &'static [u8] = include_bytes!("../../assets/feather.ttf");

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(Font::with_name("icomoon"))
        .align_x(Alignment::Center)
}

macro_rules! add_icon {
    ($name:ident, $unicode:expr) => {
        pub fn $name() -> Text<'static> {
            icon($unicode)
        }
    };
}

add_icon!(folder_icon, '\u{E950}');
add_icon!(add_folder_icon, '\u{E952}');
add_icon!(add_file_icon, '\u{E96F}');
add_icon!(image_icon, '\u{E978}');
add_icon!(settings_icon, '\u{E9DB}');
add_icon!(delete_icon, '\u{E9F6}');
