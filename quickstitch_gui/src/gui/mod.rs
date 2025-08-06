use iced::{
    Alignment, Element,
    Length::FillPortion,
    Theme,
    widget::{button, column, horizontal_rule, row, scrollable, text},
};
use icons::{folder_icon, image_icon, settings_icon, sliders_icon};
use io_section::{IOSection, IOSectionMessage};
use limit_section::{LimitSection, LimitSectionMessage};
use setting_section::{SettingSection, SettingSectionMessage};

pub mod icons;
mod io_section;
mod limit_section;
mod setting_section;

pub struct Quickstitch {
    io_section: IOSection,
    limit_section: LimitSection,
    setting_section: SettingSection,
    theme: Theme,
}

impl Default for Quickstitch {
    fn default() -> Self {
        let io_section = IOSection::default();
        let limit_section = LimitSection::new(io_section.get_output_format());
        Self {
            limit_section,
            io_section,
            setting_section: SettingSection::default(),
            theme: Theme::Light,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    IOSection(IOSectionMessage),
    LimitSection(LimitSectionMessage),
    SettingSection(SettingSectionMessage),
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
                self.io_section.view().map(Message::IOSection),
                horizontal_rule(3),
                // Image limits
                row![image_icon().size(32), text("Size Limits").size(32)].spacing(10),
                self.limit_section.view().map(Message::LimitSection),
                horizontal_rule(3),
                // Algorithm settings
                row![
                    settings_icon().size(32),
                    text("Settings (Advanced)").size(32)
                ]
                .spacing(10),
                self.setting_section.view().map(Message::SettingSection),
                horizontal_rule(3),
                // Action buttons
                row![sliders_icon().size(32), text("Actions").size(32)].spacing(10),
                row![
                    button(text("Stitch").align_x(Alignment::Center).size(20))
                        .width(FillPortion(1)),
                    button(text("Edit").align_x(Alignment::Center).size(20)).width(FillPortion(1)),
                ]
                .spacing(20),
            ]
            .spacing(20)
            .padding(20),
        )
        .into()
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::IOSection(io_message) => {
                self.io_section.update(io_message);
            }
            Message::LimitSection(limit_section_message) => {
                self.limit_section.update(limit_section_message);
            }
            Message::SettingSection(setting_section_message) => {
                self.setting_section.update(setting_section_message);
            }
        }
    }
    pub fn get_theme(&self) -> Theme {
        self.theme.clone()
    }
}
