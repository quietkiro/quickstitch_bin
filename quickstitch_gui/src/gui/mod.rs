use iced::{
    Element,
    Length::FillPortion,
    Theme,
    widget::{Space, button, column, horizontal_rule, row, scrollable, text},
};
use icons::{folder_icon, image_icon, settings_icon};
use io_section::{IOSection, IOSectionMessage};
use limit_section::{LimitSection, LimitSectionMessage};
use quickstitch::Splitpoint;
use setting_section::{SettingSection, SettingSectionMessage};

use crate::stitcher::stitcher;

pub mod icons;
pub mod io_section;
pub mod limit_section;
pub mod setting_section;

pub struct Quickstitch {
    io_section: IOSection,
    limit_section: LimitSection,
    setting_section: SettingSection,
    theme: Theme,
    splitpoints: Option<Vec<Splitpoint>>,
    stitch_error: String,
}

impl Default for Quickstitch {
    fn default() -> Self {
        let io_section = IOSection::default();
        let limit_section = LimitSection::new(io_section.output_format());
        Self {
            limit_section,
            io_section,
            setting_section: SettingSection::default(),
            theme: Theme::Light,
            splitpoints: None,
            stitch_error: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    IOSection(IOSectionMessage),
    LimitSection(LimitSectionMessage),
    SettingSection(SettingSectionMessage),
    Stitch,
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
                column![
                    button(row![
                        Space::with_width(FillPortion(1)),
                        text("Stitch").size(32),
                        Space::with_width(FillPortion(1))
                    ])
                    .on_press(Message::Stitch)
                    .width(FillPortion(1)),
                    text(&self.stitch_error).size(16).style(text::danger),
                ]
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
            Message::Stitch => {
                match stitcher(
                    self.io_section.input_type(),
                    self.io_section.input_directory(),
                    self.io_section.sort_method(),
                    self.io_section.input_files(),
                    self.io_section.ignore_unlodable(),
                    self.io_section.output_directory(),
                    self.io_section.output_format().borrow().clone(),
                    self.io_section.compression_quality(),
                    self.limit_section.width_type(),
                    self.limit_section.fixed_width(),
                    self.limit_section.max_height(),
                    self.limit_section.min_height(),
                    self.setting_section.scan_interval(),
                    self.setting_section.sensitivity(),
                    self.setting_section.debug(),
                ) {
                    Ok(splitpoints) => {
                        self.splitpoints = Some(splitpoints);
                        self.stitch_error = String::new();
                    }
                    Err(e) => self.stitch_error = e.to_string(),
                }
            }
        }
    }
    pub fn get_theme(&self) -> Theme {
        self.theme.clone()
    }
}
