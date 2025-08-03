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
    pub fn with_path(path: PathBuf) -> Self {
        Self { path }
    }
    pub fn view(&self) -> Element<ImageFileMessage> {
        row![
            text(self.path.file_name().unwrap().display().to_string()).size(20),
            button(delete_icon().size(20))
                .style(button::danger)
                .on_press(ImageFileMessage::Delete)
        ]
        .spacing(20)
        .into()
    }
}
