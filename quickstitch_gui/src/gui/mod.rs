use iced::{
    Element, Theme,
    widget::{column, horizontal_rule, row, text},
};
use icons::{folder_icon, image_icon, settings_icon};
use io_section::{IOMessage, IOSection};

pub mod icons;
mod io_section;

#[derive(Default)]
pub struct Quickstitch {
    io_section: IOSection,
    theme: Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    IOMessage(IOMessage),
}

impl Quickstitch {
    pub fn view(&self) -> Element<Message> {
        // Sizing chart:
        // H1 - 32
        // H2 - 24
        // H3 - 20
        // Text 16
        column![
            // Input/Output directories
            row![folder_icon().size(32), text("I/O").size(32)].spacing(10),
            self.io_section.view().map(Message::IOMessage),
            horizontal_rule(3),
            // Image limits
            row![image_icon().size(32), text("Size Limits").size(32)].spacing(10),
            horizontal_rule(3),
            // Algorithm settings
            row![settings_icon().size(32), text("Settings").size(32)].spacing(10),
            horizontal_rule(3),
            // Action buttons
            // Preview/Export
        ]
        .spacing(20)
        .padding(20)
        .into()
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::IOMessage(io_message) => {
                self.io_section.update(io_message);
            }
        }
    }
    pub fn get_theme(&self) -> Theme {
        self.theme.clone()
    }
}
