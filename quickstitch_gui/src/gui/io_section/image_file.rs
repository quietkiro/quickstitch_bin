use std::path::PathBuf;

use iced::{
    Element,
    widget::{button, row, text},
};

use crate::gui::icons::delete_icon;

pub struct ImageFile {
    path: PathBuf,
}

#[derive(Clone, Debug)]
pub enum ImageFileMessage {
    Delete,
}

impl ImageFile {
    pub fn view(&self) -> Element<ImageFileMessage> {
        row![
            text(self.path.file_name().unwrap().display().to_string()),
            button(delete_icon())
                .style(button::danger)
                .on_press(ImageFileMessage::Delete)
        ]
        .into()
    }
}
