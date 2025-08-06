use std::{cell::RefCell, rc::Rc};

use iced::{
    Element,
    Length::FillPortion,
    widget::{column, row, text, text_input},
};

use crate::gui::io_section::ImageFormat;

#[derive(Default)]
pub struct PixelField {
    title: String,
    field: String,
    hint: String,
    number: Option<usize>,
    output_format: Rc<RefCell<ImageFormat>>,
}

#[derive(Debug, Clone)]
pub enum PixelFieldMessage {
    UpdateField(String),
}

impl PixelField {
    pub fn number(&self) -> Option<usize> {
        self.number
    }
    pub fn new<S: AsRef<str>>(
        title: S,
        hint: S,
        prefill: Option<usize>,
        output_format: Rc<RefCell<ImageFormat>>,
    ) -> Self {
        Self {
            title: title.as_ref().to_string(),
            hint: hint.as_ref().to_string(),
            number: prefill,
            field: if let Some(num) = prefill {
                num.to_string()
            } else {
                String::new()
            },
            output_format,
            ..Default::default()
        }
    }
    pub fn view(&self) -> Element<PixelFieldMessage> {
        row![
            column![
                text(&self.title).size(20),
                text(format!(
                    "Max: {} pixels",
                    self.output_format.borrow().limit()
                ))
                .size(16)
                .style(text::secondary),
            ]
            .width(FillPortion(1)),
            text_input(&self.hint, &self.field)
                .width(FillPortion(1))
                .on_input(PixelFieldMessage::UpdateField)
        ]
        .into()
    }
    pub fn update(&mut self, message: PixelFieldMessage) {
        match message {
            PixelFieldMessage::UpdateField(field) => {
                if let Ok(num) = field.parse::<usize>()
                    && num < self.output_format.borrow().limit()
                {
                    self.field = num.to_string();
                    self.number = Some(num);
                } else if field.is_empty() {
                    self.field = String::new();
                    self.number = None;
                }
            }
        }
    }
}
