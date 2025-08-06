use std::{cell::RefCell, path::PathBuf, rc::Rc};

use iced::{
    Element,
    Length::FillPortion,
    widget::{button, column, container, radio, row, scrollable, text, text_input},
};
use image_file::{ImageFile, ImageFileMessage};
use rfd::FileDialog;

use super::icons::{add_file_icon, add_folder_icon, folder_icon, image_icon};

mod image_file;

pub struct IOSection {
    input_type: InputType,
    input_directory: Option<PathBuf>,
    sort_method: SortMethod,
    input_files: Vec<ImageFile>,
    output_directory: Option<PathBuf>,
    output_format: Rc<RefCell<ImageFormat>>,
    quality_field: String,
    quality: Option<u8>,
}

impl Default for IOSection {
    fn default() -> Self {
        Self {
            input_type: InputType::default(),
            input_directory: None,
            sort_method: SortMethod::default(),
            input_files: vec![],
            output_directory: None,
            output_format: Rc::default(),
            quality_field: "100".to_string(),
            quality: Some(100),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub enum InputType {
    #[default]
    Directory,
    Images,
}

#[derive(Default, PartialEq, Eq, Copy, Clone, Debug)]
pub enum SortMethod {
    #[default]
    Natural,
    Logical,
}

#[derive(Default, Clone, Debug, PartialEq, Copy)]
pub enum ImageFormat {
    #[default]
    JPEG,
    WebP,
    PNG,
}

impl ImageFormat {
    pub fn limit(&self) -> u32 {
        match self {
            ImageFormat::JPEG => 65_535,
            ImageFormat::WebP => 16_383,
            ImageFormat::PNG => u32::MAX,
        }
    }
}

#[derive(Debug, Clone)]
pub enum IOSectionMessage {
    ImageFileMessage(usize, ImageFileMessage),
    SetInputType(InputType),
    SetInputDirectory,
    SetOutputDirectory,
    AddImage,
    SetOutputFormat(ImageFormat),
    SetSortMethod(SortMethod),
    SetQualityField(String),
}

impl IOSection {
    pub fn get_output_format(&self) -> Rc<RefCell<ImageFormat>> {
        self.output_format.clone()
    }
    pub fn view(&self) -> Element<IOSectionMessage> {
        let select_dir_field =
            |set_dir_message, dir: &Option<PathBuf>| -> Element<IOSectionMessage> {
                match &dir {
                    Some(dir) => column![
                        scrollable(text(dir.display().to_string()).size(20))
                            .horizontal()
                            .spacing(5),
                        button(
                            row![
                                add_folder_icon().size(20),
                                text("Change Directory").size(20)
                            ]
                            .spacing(10)
                        )
                        .on_press(set_dir_message)
                        .style(button::danger)
                    ]
                    .spacing(10)
                    .into(),
                    None => button(
                        row![
                            add_folder_icon().size(20),
                            text("Select Directory").size(20)
                        ]
                        .spacing(10),
                    )
                    .on_press(set_dir_message)
                    .into(),
                }
            };

        let images: Element<_> = match self.input_type {
            InputType::Directory => column![
                row![
                    column![
                        text("Input Directory").size(20),
                        text("Folder from which the images will be taken")
                            .size(16)
                            .style(text::secondary)
                    ]
                    .width(FillPortion(1)),
                    container(select_dir_field(
                        IOSectionMessage::SetInputDirectory,
                        &self.input_directory
                    ))
                    .width(FillPortion(1)),
                ]
                .spacing(20),
                row![
                    column![
                        text("Image Sorting").size(20),
                        text("Choose how the input images are sorted")
                            .size(16)
                            .style(text::secondary)
                    ]
                    .width(FillPortion(1)),
                    column![
                        radio(
                            "Natural - [\"10\", \"11\", \"8\", \"9\"]",
                            SortMethod::Natural,
                            Some(self.sort_method),
                            IOSectionMessage::SetSortMethod
                        )
                        .size(20),
                        radio(
                            "Logical - [\"8\", \"9\", \"10\", \"11\"]",
                            SortMethod::Logical,
                            Some(self.sort_method),
                            IOSectionMessage::SetSortMethod
                        )
                        .size(20)
                    ]
                    .spacing(10)
                    .width(FillPortion(1))
                ]
                .spacing(20)
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
                if !self.input_files.is_empty() {
                    column![
                        column(self.input_files.iter().enumerate().map(|(i, file)| {
                            file.view()
                                .map(move |a| IOSectionMessage::ImageFileMessage(i, a))
                        }))
                        .spacing(10),
                        button(
                            row![add_file_icon().size(20), text("Add Image").size(20)].spacing(10)
                        )
                        .on_press(IOSectionMessage::AddImage)
                    ]
                    .spacing(10)
                    .width(FillPortion(1))
                } else {
                    column![
                        button(row![add_file_icon().size(20), text("Add Image").size(20)])
                            .on_press(IOSectionMessage::AddImage)
                    ]
                    .width(FillPortion(1))
                }
            ]
            .spacing(20)
            .into(),
        };

        let image_format_button = |name, filetype, curr_filetype: Rc<RefCell<ImageFormat>>| {
            button(text(name).size(20))
                .style(if filetype == *curr_filetype.borrow() {
                    button::primary
                } else {
                    button::text
                })
                .on_press(IOSectionMessage::SetOutputFormat(filetype))
        };

        let mut output_format = column![
            row![
                column![
                    text("Output Format").size(20),
                    text("File format to use when exporting stitched images")
                        .size(16)
                        .style(text::secondary)
                ]
                .width(FillPortion(1)),
                row![
                    image_format_button("JPEG", ImageFormat::JPEG, self.output_format.clone()),
                    image_format_button("WebP", ImageFormat::WebP, self.output_format.clone()),
                    image_format_button("PNG", ImageFormat::PNG, self.output_format.clone())
                ]
                .spacing(10)
                .width(FillPortion(1))
            ]
            .spacing(20)
        ]
        .spacing(20);

        if *self.output_format.borrow() == ImageFormat::JPEG {
            output_format = output_format.push(
                row![
                    column![
                        text("Compression Quality").size(20),
                        text("Compression quality to use when exporting JPEG (1-100)")
                            .size(16)
                            .style(text::secondary)
                    ]
                    .width(FillPortion(1)),
                    text_input("e.g. 100", &self.quality_field)
                        .width(FillPortion(1))
                        .on_input(IOSectionMessage::SetQualityField),
                ]
                .spacing(20),
            );
        }

        // Final UI

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
                    button(row![folder_icon().size(20), text("Directory").size(20)].spacing(10))
                        .on_press(IOSectionMessage::SetInputType(InputType::Directory))
                        .style(match self.input_type {
                            InputType::Directory => button::primary,
                            InputType::Images => button::text,
                        }),
                    button(row![image_icon().size(20), text("Images").size(20)].spacing(10))
                        .on_press(IOSectionMessage::SetInputType(InputType::Images))
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
                container(select_dir_field(
                    IOSectionMessage::SetOutputDirectory,
                    &self.output_directory
                ))
                .width(FillPortion(1))
            ]
            .spacing(20),
            output_format,
        ]
        .spacing(20)
        .into()
    }
    pub fn update(&mut self, message: IOSectionMessage) {
        match message {
            IOSectionMessage::ImageFileMessage(i, message) => {
                match message {
                    ImageFileMessage::Delete => self.input_files.remove(i),
                };
            }
            IOSectionMessage::SetInputDirectory => {
                if let Some(dir) = FileDialog::new().pick_folder() {
                    self.input_directory = Some(dir);
                }
            }
            IOSectionMessage::SetOutputDirectory => {
                if let Some(dir) = FileDialog::new().pick_folder() {
                    self.output_directory = Some(dir)
                }
            }
            IOSectionMessage::SetInputType(input_type) => self.input_type = input_type,
            IOSectionMessage::AddImage => {
                if let Some(file) = FileDialog::new()
                    .add_filter("Image (webp, png, jpeg)", &["webp", "png", "jpeg", "jpg"])
                    .pick_file()
                {
                    self.input_files.push(ImageFile::with_path(file));
                }
            }
            IOSectionMessage::SetOutputFormat(output_format) => {
                *self.output_format.borrow_mut() = output_format
            }
            IOSectionMessage::SetSortMethod(sort_method) => self.sort_method = sort_method,
            IOSectionMessage::SetQualityField(quality_field) => {
                if let Ok(num) = quality_field.parse::<u8>()
                    && num <= 100
                    && num > 0
                {
                    self.quality_field = num.to_string();
                    self.quality = Some(num);
                } else if quality_field.is_empty() {
                    self.quality_field = String::new();
                    self.quality = None;
                }
            }
        }
    }
}
