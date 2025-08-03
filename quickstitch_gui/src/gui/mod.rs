use iced::{
    Element, Theme,
    widget::{column, horizontal_rule, row, scrollable, text},
};
use icons::{folder_icon, image_icon, settings_icon, sliders_icon};
use io_section::{IOMessage, IOSection};
use limit_section::{LimitSection, LimitSectionMessage};

pub mod icons;
mod io_section;
mod limit_section;

pub struct Quickstitch {
    io_section: IOSection,
    limit_section: LimitSection,
    theme: Theme,
}

impl Default for Quickstitch {
    fn default() -> Self {
        let io_section = IOSection::default();
        let limit_section = LimitSection::new(io_section.get_output_format());
        Self {
            limit_section,
            io_section,
            theme: Theme::Light,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    IOMessage(IOMessage),
    LimitSectionMessage(LimitSectionMessage),
}

impl Quickstitch {
    pub fn view(&self) -> Element<Message> {
        // Sizing chart:
        // H1 - 32
        // H2 - 24
        // H3 - 20
        // Text 16
        scrollable(
            column![
                // Input/Output directories
                row![folder_icon().size(32), text("I/O").size(32)].spacing(10),
                self.io_section.view().map(Message::IOMessage),
                horizontal_rule(3),
                // Image limits
                row![image_icon().size(32), text("Size Limits").size(32)].spacing(10),
                self.limit_section.view().map(Message::LimitSectionMessage),
                horizontal_rule(3),
                // Algorithm settings
                row![settings_icon().size(32), text("Settings").size(32)].spacing(10),
                horizontal_rule(3),
                // Action buttons
                row![sliders_icon().size(32), text("Actions").size(32)].spacing(10),
                horizontal_rule(3),
            ]
            .spacing(20)
            .padding(20),
        )
        .into()
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::IOMessage(io_message) => {
                self.io_section.update(io_message);
            }
            Message::LimitSectionMessage(limit_section_message) => {
                self.limit_section.update(limit_section_message);
            }
        }
    }
    pub fn get_theme(&self) -> Theme {
        self.theme.clone()
    }
}
