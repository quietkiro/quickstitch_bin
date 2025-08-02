use std::path::PathBuf;

use iced::{
    Element,
    Length::FillPortion,
    widget::{button, column, container, row, scrollable, text},
};
use image_file::{ImageFile, ImageFileMessage};
use rfd::FileDialog;

mod image_file;

#[derive(Default)]
pub struct IOSection {
    input_type: InputType,
    input_directory: Option<PathBuf>,
    input_files: Vec<ImageFile>,
    output_directory: Option<PathBuf>,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub enum InputType {
    #[default]
    Directory,
    Images,
}

#[derive(Debug, Clone)]
pub enum IOMessage {
    ImageFileMessage(usize, ImageFileMessage),
    SetInputType(InputType),
    SetInputDirectory,
    SetOutputDirectory,
}

impl IOSection {
    pub fn view(&self) -> Element<IOMessage> {
        let select_dir_field = |set_dir_message| -> Element<IOMessage> {
            match &self.input_directory {
                Some(dir) => column![
                    scrollable(
                        text(dir.display().to_string()).size(20) // .wrapping(text::Wrapping::None)
                    )
                    .horizontal()
                    .spacing(5),
                    button(text("Change Directory").size(20))
                        .on_press(set_dir_message)
                        .style(button::danger)
                ]
                .spacing(10)
                .into(),
                None => button(text("Select Directory").size(20))
                    .on_press(set_dir_message)
                    .into(),
            }
        };

        let images: Element<_> = match self.input_type {
            InputType::Directory => row![
                column![
                    text("Input Directory").size(20),
                    text("Folder from which the images will be taken")
                        .size(16)
                        .style(text::secondary)
                ]
                .width(FillPortion(1)),
                container(select_dir_field(IOMessage::SetInputDirectory)).width(FillPortion(1)),
            ]
            .spacing(20)
            .into(),
            InputType::Images => row![
                column![
                    text("Input Images").size(20),
                    text("Choose which images will be used")
                        .size(16)
                        .style(text::secondary)
                ]
                .width(FillPortion(1)),
                column(self.input_files.iter().enumerate().map(|(i, file)| {
                    file.view().map(move |a| IOMessage::ImageFileMessage(i, a))
                }))
                .width(FillPortion(1))
            ]
            .spacing(20)
            .into(),
        };
        column![
            row![
                column![
                    text("Input Type").size(20),
                    text("Define the image source type")
                        .size(16)
                        .style(text::secondary)
                ]
                .width(FillPortion(1)),
                row![
                    button(text("Directory").size(20))
                        .on_press(IOMessage::SetInputType(InputType::Directory))
                        .style(match self.input_type {
                            InputType::Directory => button::primary,
                            InputType::Images => button::text,
                        }),
                    button(text("Images").size(20))
                        .on_press(IOMessage::SetInputType(InputType::Images))
                        .style(match self.input_type {
                            InputType::Directory => button::text,
                            InputType::Images => button::primary,
                        })
                ]
                .spacing(10)
                .width(FillPortion(1)),
            ]
            .spacing(20),
            images,
            row![
                column![
                    text("Output Directory").size(20),
                    text("Folder to which the stitched image(s) will be saved")
                        .size(16)
                        .style(text::secondary)
                ]
                .width(FillPortion(1)),
                container(select_dir_field(IOMessage::SetOutputDirectory)).width(FillPortion(1))
            ]
            .spacing(20),
        ]
        .spacing(20)
        .into()
    }
    pub fn update(&mut self, message: IOMessage) {
        match message {
            IOMessage::ImageFileMessage(i, message) => {
                match message {
                    ImageFileMessage::Delete => self.input_files.remove(i),
                };
            }
            IOMessage::SetInputDirectory => {
                if let Some(dir) = FileDialog::new().pick_folder() {
                    self.input_directory = Some(dir);
                }
            }
            IOMessage::SetOutputDirectory => {
                if let Some(dir) = FileDialog::new().pick_folder() {
                    self.output_directory = Some(dir)
                }
            }
            IOMessage::SetInputType(input_type) => self.input_type = input_type,
        }
    }
}
