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

pub fn folder_icon() -> Text<'static> {
    icon('\u{E950}')
}

pub fn image_icon() -> Text<'static> {
    icon('\u{E978}')
}

pub fn settings_icon() -> Text<'static> {
    icon('\u{E9DB}')
}

pub fn delete_icon() -> Text<'static> {
    icon('\u{E9F6}')
}
