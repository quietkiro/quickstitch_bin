use std::{cell::RefCell, rc::Rc};

use iced::{
    Element,
    Length::FillPortion,
    widget::{column, radio, row, text},
};
use pixel_field::{PixelField, PixelFieldMessage};

use super::io_section::ImageFormat;

mod pixel_field;

#[derive(Default)]
pub struct LimitSection {
    width_type: WidthType,
    fixed_width: PixelField,
    max_height: PixelField,
    min_height: PixelField,
}

#[derive(Default, PartialEq, Eq, Copy, Clone, Debug)]
pub enum WidthType {
    Auto,
    #[default]
    Fixed,
}

#[derive(Clone, Debug)]
pub enum LimitSectionMessage {
    SetWidthType(WidthType),
    FixedWidthMessage(PixelFieldMessage),
    MaxHeightMessage(PixelFieldMessage),
    MinHeightMessage(PixelFieldMessage),
}

impl LimitSection {
    pub fn width_type(&self) -> WidthType {
        self.width_type
    }
    pub fn fixed_width(&self) -> Option<u32> {
        self.fixed_width.number().map(|num| num as u32)
    }
    pub fn max_height(&self) -> Option<usize> {
        self.max_height.number()
    }
    pub fn min_height(&self) -> Option<usize> {
        self.min_height.number()
    }
    pub fn new(output_format: Rc<RefCell<ImageFormat>>) -> Self {
        Self {
            fixed_width: PixelField::new(
                "Output Image Width",
                "e.g. 800",
                Some(800),
                output_format.clone(),
            ),
            max_height: PixelField::new(
                "Max Output Height",
                "e.g. 15000",
                Some(15000),
                output_format.clone(),
            ),
            min_height: PixelField::new(
                "Min Output Height",
                "e.g. 10000",
                Some(10000),
                output_format.clone(),
            ),
            ..Default::default()
        }
    }
    pub fn view(&self) -> Element<LimitSectionMessage> {
        let mut width_settings = column![
            row![
                column![
                    text("Output Width Type").size(20),
                    text("Define how the stitched image width is chosen.")
                        .size(16)
                        .style(text::secondary),
                ]
                .width(FillPortion(1)),
                column![
                    radio(
                        "Fixed - Set a fixed width",
                        WidthType::Fixed,
                        Some(self.width_type),
                        LimitSectionMessage::SetWidthType
                    )
                    .size(20),
                    radio(
                        "Auto - Use the smallest width of the input images",
                        WidthType::Auto,
                        Some(self.width_type),
                        LimitSectionMessage::SetWidthType
                    )
                    .size(20)
                ]
                .spacing(10)
                .width(FillPortion(1))
            ]
            .spacing(20)
        ]
        .spacing(20);
        if self.width_type == WidthType::Fixed {
            width_settings = width_settings.push(
                self.fixed_width
                    .view()
                    .map(LimitSectionMessage::FixedWidthMessage),
            );
        }

        // Final UI

        column![
            width_settings,
            self.max_height
                .view()
                .map(LimitSectionMessage::MaxHeightMessage),
            self.min_height
                .view()
                .map(LimitSectionMessage::MinHeightMessage)
        ]
        .spacing(20)
        .into()
    }
    pub fn update(&mut self, message: LimitSectionMessage) {
        match message {
            LimitSectionMessage::SetWidthType(width_type) => self.width_type = width_type,
            LimitSectionMessage::FixedWidthMessage(msg) => self.fixed_width.update(msg),
            LimitSectionMessage::MaxHeightMessage(msg) => self.max_height.update(msg),
            LimitSectionMessage::MinHeightMessage(msg) => self.min_height.update(msg),
        }
    }
}
