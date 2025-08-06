use iced::{
    Element,
    Length::FillPortion,
    widget::{column, container, row, text, text_input, toggler},
};

pub struct SettingSection {
    debug: bool,
    scan_interval_field: String,
    scan_interval: Option<u32>,
    sensitivity_field: String,
    sensitivity: Option<u8>,
}

impl Default for SettingSection {
    fn default() -> Self {
        Self {
            debug: false,
            scan_interval_field: "5".to_string(),
            scan_interval: Some(5),
            sensitivity_field: "255".to_string(),
            sensitivity: Some(255),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SettingSectionMessage {
    SetDebugMode(bool),
    SetScanInterval(String),
    SetSensitivity(String),
}

impl SettingSection {
    pub fn view(&self) -> Element<SettingSectionMessage> {
        column![
            row![
                column![
                    text("Scan Interval").size(20),
                    text("Interval at which lines of pixels are scanned")
                        .size(16)
                        .style(text::secondary)
                ].width(FillPortion(1)),
                text_input("e.g. 5", &self.scan_interval_field)
                    .on_input(SettingSectionMessage::SetScanInterval)
                    .size(20)
                    .width(FillPortion(1))
            ]
            .spacing(20),
            row![
                column![
                    text("Sensitivity").size(20),
                    text("Sensitivity for determining when pixel line should be used as splitpoint.(0-255)").size(16).style(text::secondary)
                ].width(FillPortion(1)),
                text_input("e.g. 255", &self.sensitivity_field)
                    .on_input(SettingSectionMessage::SetSensitivity)
                    .size(20)
                    .width(FillPortion(1))
            ].spacing(20),
            row![
                column![
                    text("Debug Mode").size(20),
                    text("Show colored lines in output images.")
                        .size(16)
                        .style(text::secondary),
                ].width(FillPortion(1)),
                container(
                    toggler(self.debug)
                        .on_toggle(SettingSectionMessage::SetDebugMode)
                        .size(20)
                ).width(FillPortion(1))
            ]
            .spacing(20),
        ]
        .spacing(20)
        .into()
    }
    pub fn update(&mut self, message: SettingSectionMessage) {
        match message {
            SettingSectionMessage::SetDebugMode(mode) => self.debug = mode,
            SettingSectionMessage::SetScanInterval(field) => {
                if let Ok(num) = field.parse::<u32>() {
                    self.scan_interval = Some(num);
                    self.scan_interval_field = num.to_string();
                } else if field.is_empty() {
                    self.scan_interval = None;
                    self.scan_interval_field = String::new();
                }
            }
            SettingSectionMessage::SetSensitivity(field) => {
                if let Ok(num) = field.parse::<u8>() {
                    self.sensitivity = Some(num);
                    self.sensitivity_field = num.to_string();
                } else if field.is_empty() {
                    self.sensitivity = None;
                    self.sensitivity_field = String::new();
                }
            }
        }
    }
}
